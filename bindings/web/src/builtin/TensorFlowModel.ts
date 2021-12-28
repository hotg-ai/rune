import * as tf from "@tensorflow/tfjs";
import { InferenceModel, ModelTensorInfo, Tensor } from "@tensorflow/tfjs-core";
import { Model } from "../Runtime";
import Shape from "../Shape";

// Explicitly pull in the CPU backend
import '@tensorflow/tfjs-backend-cpu';
import { toTypedArray } from "../helpers";

/**
 * A TensorFlow model.
 */
export class TensorFlowModel implements Model {
    private model: InferenceModel;

    constructor(model: InferenceModel) {
        this.model = model;
    }

    transform(inputArray: Uint8Array[], inputDimensions: Shape[], outputArray: Uint8Array[], outputDimensions: Shape[]): void {
        const inputs = toTensors(inputArray, inputDimensions);
        const output = this.model.predict(inputs, {});

        if (Array.isArray(output)) {
            output.forEach((tensor, i) => outputArray[i].set(tensor.dataSync()));
        } else if (output instanceof Tensor) {
            var dest = outputArray[0];
            var out = output.dataSync();
            dest.set(out);
        } else {
            const namesToIndices: Record<string, number> = {};
            this.model.outputs.forEach((info, i) => namesToIndices[info.name] = i);

            for (const name in output) {
                const tensor = output[name];
                const index = namesToIndices[name];
                outputArray[index].set(tensor.dataSync());
            }
        }
    }

    get inputs(): ModelTensorInfo[] {
        return this.model.inputs;
    }

    get outputs(): ModelTensorInfo[] {
        return this.model.outputs;
    }
}

function toTensors(buffers: Uint8Array[], shapes: Shape[]): Tensor[] {
    const tensors = [];

    for (let i = 0; i < buffers.length; i++) {
        const buffer = buffers[i];
        const shape = shapes[i];
        const arr = toTypedArray(shape.type, buffer);
        tensors.push(tf.tensor(arr, shape.dimensions));
    }

    return tensors;
}

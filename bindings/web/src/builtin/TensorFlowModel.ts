import * as tf from "@tensorflow/tfjs-core";
import { InferenceModel, ModelTensorInfo, Tensor } from "@tensorflow/tfjs-core";
import { Model } from "../Runtime";
import Shape from "../Shape";
import { toTypedArray } from "../helpers";

// Registers the default backends
import "@tensorflow/tfjs";

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

    get inputs(): Shape[] {
        return this.model.inputs.map(toShape);
    }

    get outputs(): Shape[] {
        return this.model.outputs.map(toShape);
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

function toShape({ dtype, shape, tfDtype }: ModelTensorInfo): Shape {
    return new Shape(
        tfDtype || dtype,
        // Note: The TypeScript declarations actually lie here. Depending on the
        // actual model, our "shape" may either be an Array<number> or a
        // Array<number|null>.
        //
        // As a best effort, we try to filter out the dimensions with unknown
        // lengths (null) and fall back to the empty array if no shape was
        // provided at all.
        shape?.filter(s => typeof s === "number") ?? [],
    );
}

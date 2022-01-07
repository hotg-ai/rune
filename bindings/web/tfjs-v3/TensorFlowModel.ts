import * as tf from "@tensorflow/tfjs-core";
import { InferenceModel, ModelTensorInfo, Tensor } from "@tensorflow/tfjs-core";
import { Model } from "../Runtime";
import Shape from "../Shape";
import RuneTensor from "../Tensor";

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
            output.forEach((tensor, i) => { 
                var out = tensor.dataSync();
                outputArray[i].set(new Uint8Array(out.buffer).slice(0,outputArray[i].length)); 
            });
        } else if (output instanceof Tensor) {
            var dest = outputArray[0];
            var out = output.dataSync();
            dest.set(new Uint8Array(out.buffer).slice(0, dest.length));
        } else {
            const namesToIndices: Record<string, number> = {};
            this.model.outputs.forEach((info, i) => namesToIndices[info.name] = i);

            for (const name in output) {
                const tensor = output[name];
                const index = namesToIndices[name];
                var out = tensor.dataSync();
                outputArray[index].set(new Uint8Array(out.buffer).slice(0, outputArray[index].length));
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
        const tensor = new RuneTensor(shape, buffer);
        tensors.push(toTensorFlowTensor(tensor));
    }

    return tensors;
}

function toTensorFlowTensor(tensor: RuneTensor): Tensor {
    const { elementType, dimensions } = tensor;
    const dims = [...dimensions];

    switch (elementType) {
        case "f32":
            const floats = tensor.asTypedArray(elementType);
            return tf.tensor(floats, dims, "float32");

        // This is kinda annoying because it's just copy/paste, but we
        // can't combine them because TypeScript errors with the following:
        //
        //   The call would have succeeded against this implementation, but
        //   implementation signatures of overloads are not externally
        //   visible.

        case "u8":
            const u8s = tensor.asTypedArray(elementType);
            return tf.tensor(Array.from(u8s), dims, "int32");
        case "u16":
            const u16s = tensor.asTypedArray(elementType);
            return tf.tensor(Array.from(u16s), dims, "int32");
        case "u32":
            const u32s = tensor.asTypedArray(elementType);
            return tf.tensor(Array.from(u32s), dims, "int32");
        case "i8":
            const i8s = tensor.asTypedArray(elementType);
            return tf.tensor(Array.from(i8s), dims, "int32");
        case "i16":
            const i16s = tensor.asTypedArray(elementType);
            return tf.tensor(Array.from(i16s), dims, "int32");
        case "i32":
            const i32s = tensor.asTypedArray(elementType);
            return tf.tensor(Array.from(i32s), dims, "int32");

        default:
            throw new Error(`Unable to convert a ${tensor.shape.toString()} to a tfjs tensor`);
    }
}

/**
 * The actual type we receive when given a ModelTensorInfo as part of an
 * InferenceModel's "inputs" or "outputs" field.
 */
type ActualModelTensorInfo = {
    name?: string;
    shape?: Array<number | null>;
    dtype?: ModelTensorInfo["dtype"];
    tfDtype?: string;
};

/**
 * Convert model tensor information to a Shape that Rune can more easily
 * consume.
 */
function toShape({ dtype, shape, tfDtype }: ActualModelTensorInfo): Shape {
    const cleanedShape = [];

    // As a best effort, we try to filter out the dimensions with unknown
    // lengths (null or -1) and fall back to the empty array if no shape was
    // provided at all.
    if (shape) {
        for (const dimension of shape) {
            if (typeof dimension === "number" && dimension >= 0) {
                cleanedShape.push(dimension);
            }
        }
    }

    return new Shape(
        tfDtype || dtype || "unknown",
        cleanedShape,
    );
}

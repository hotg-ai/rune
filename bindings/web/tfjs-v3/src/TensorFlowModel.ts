import * as tf from "@tensorflow/tfjs-core";
import { InferenceModel, ModelTensorInfo, NamedTensorMap, Tensor } from "@tensorflow/tfjs-core";
import { Shape, Tensor as RuneTensor } from "@hotg-ai/rune";
import { Model } from "@hotg-ai/rune/Runtime";

/**
 * A TensorFlow model.
 */
export class TensorFlowModel implements Model {
  private model: InferenceModel;

  constructor(model: InferenceModel) {
    this.model = model;
  }

  transform(
    inputArray: Uint8Array[],
    inputDimensions: Shape[],
    outputArray: Uint8Array[],
    outputDimensions: Shape[]
  ): void {
    tf.tidy(() => {
      const inputs = toTensors(inputArray, inputDimensions);
      const result = this.model.predict(inputs, {});

      let outputs: Tensor[];

      if (Array.isArray(result)) {
        outputs = result;
      } else if (result instanceof Tensor) {
        outputs = [result];
      } else {
        const names = this.model.outputs.map(info => info.name);
        outputs = namedTensorArray(names, result);
      }

      if (outputs.length != outputArray.length) {
        throw new Error(`The model returned ${outputs.length} tensors, but the Rune expects ${outputArray.length}`);
      }

      outputs.forEach((tensor, i) => {
        var out = tensor.dataSync();
        const shape = outputDimensions[i];
        const typedArray = typedArrayFromTFJS(out, shape);
        const bytes = new Uint8Array(typedArray.buffer, typedArray.byteOffset, typedArray.byteLength);
        outputArray[i].set(bytes);
      });
    });
  }

  get inputs(): Shape[] {
    return this.model.inputs.map(toShape);
  }

  get outputs(): Shape[] {
    return this.model.outputs.map(toShape);
  }
}

function namedTensorArray(names: string[], result: NamedTensorMap): Tensor[] {
  const outputs = [];

  for (const name of names) {
    if (name in result) {
      outputs.push(result[name]);
    } else {
      const commaSeparatedNames = Object.keys(result);
      throw new Error(`Tried to get the \"${name}\" output, but inference returned \"${JSON.stringify(commaSeparatedNames)}\"`);
    }
  }

  return outputs;
}

const TensorFlowToRustDataTypes: Partial<Record<Tensor["dtype"], Array<keyof typeof Shape.ByteSize>>> = {
  float32: ["f32"],
  int32: ["i32", "i16", "i8", "u8"],
};

export function assertSameShape(tensor: Tensor, shape: Shape) {
  const actualDimensions: number[] = tensor.shape;

  if (actualDimensions.toString() != shape.dimensions.toString()) {
    throw new Error(`Expected a ${shape}, but found a tensor of ${actualDimensions}`);
  }

  const matchingDataTypes: Partial<Record<string, string[]>> = TensorFlowToRustDataTypes;
  const compatibleShapes = matchingDataTypes[tensor.dtype];

  if (!compatibleShapes) {
    throw new Error(`Rune is unable to handle ${tensor.dtype} tensors`);
  }
  else if (!compatibleShapes.includes(shape.type)) {
    throw new Error(`A ${shape.type} tensor isn't compatible with ${compatibleShapes.join("or")}`);
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
      throw new Error(
        `Unable to convert a ${tensor.shape.toString()} to a tfjs tensor`
      );
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

  return new Shape(tfDtype || dtype || "unknown", cleanedShape);
}

/**
 * Convert the array buffer returned by tfjs's tensor.dataSync() into a typed
 * array with the correct type.
 * @param tensorData The raw tensor data.
 * @param shape The shape we expect.
 * @returns
 */
function typedArrayFromTFJS(tensorData: Float32Array | Int32Array | Uint8Array, shape: Shape): ArrayBufferView {
  switch (shape.type) {
    case "u8":
      if (tensorData instanceof Int32Array) {
        return new Uint8Array(tensorData);
      }
      break;
    case "i8":
      if (tensorData instanceof Int32Array) {
        return new Int8Array(tensorData);
      }
      break;
    case "u16":
      if (tensorData instanceof Int32Array) {
        return new Uint16Array(tensorData);
      }
      break;
    case "i16":
      if (tensorData instanceof Int32Array) {
        return new Int16Array(tensorData);
      }
      break;
    case "u32":
      throw new Error("TODO: Figure out whether converting a tfjs Int32Array to a tensor of u32s is valid");
    case "i32":
      if (tensorData instanceof Int32Array) {
        return new Int32Array(tensorData);
      }
      break;
    case "f32":
      return tensorData;
  }

  throw new Error(`Unable to convert a tfjs tensor of ${tensorData.constructor.name} to a ${shape.toString()}`);
}

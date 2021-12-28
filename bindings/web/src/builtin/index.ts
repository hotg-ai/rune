export { RandomCapability } from "./RandomCapability";
export { WebcamCapability } from "./WebcamCapability";
export { TensorFlowModel } from "./TensorFlowModel";
export { loadTensorFlowJS } from "./tfjs";
export { loadTensorFlowLite } from "./tflite";

/**
 * Mimetypes for the model formats known by rune.
 */
export const mimetypes = {
    tflite: "application/tflite-model",
    tensorflow: "application/tf-model",
    tfjs: "application/tfjs-model",
    onnx: "application/onnx-model",
} as const;

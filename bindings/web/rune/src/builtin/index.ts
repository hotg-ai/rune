export { RandomCapability } from "./RandomCapability";
export { WebcamCapability } from "./WebcamCapability";

/**
 * Mimetypes for the model formats known by rune.
 */
export const mimetypes = {
    tflite: "application/tflite-model",
    tensorflow: "application/tf-model",
    tfjs: "application/tfjs-model",
    onnx: "application/onnx-model",
} as const;

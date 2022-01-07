export { TensorFlowModel } from "./TensorFlowModel";

import { loadTFLiteModel } from "@tensorflow/tfjs-tflite";
import { TensorFlowModel } from "./TensorFlowModel";

/**
 * Load a TensorFlow model from the contents of a tflite file.
 */
export async function loadTensorFlowLite(buffer: ArrayBuffer): Promise<TensorFlowModel> {
    const model = await loadTFLiteModel(buffer);
    return new TensorFlowModel(model);
}

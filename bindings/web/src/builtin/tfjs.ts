import * as tf from "@tensorflow/tfjs";
import { unzip } from 'unzipit';
import { TensorFlowModel } from "./TensorFlowModel";
import { TensorFlowZipRequest } from "./TensorFlowZipRequest";

/**
 * Load a TensorFlow model from a tf.js model that has been collected into a
 * single zip archive.
 */
export async function loadTensorFlowJS(buffer: ArrayBuffer): Promise<TensorFlowModel> {
    let json: ArrayBuffer = new ArrayBuffer(0);
    let weights = new Map<string, ArrayBuffer>();
    const { entries } = await unzip(buffer);

    for (const [name, entry] of Object.entries(entries)) {
        console.log(name, entry.size);
        const arrayBuffer = await entries[name].arrayBuffer();
        console.log(name.split('.').pop());
        if (name.split('.').pop() == "json") {
            json = arrayBuffer;
            const decoder = new TextDecoder("utf-8");
            let decoded = decoder.decode(arrayBuffer);
            var jsonResult = JSON.parse(decoded);
            console.log(jsonResult)
        } else if (name.split('.').pop() == "bin") {
            weights.set(name.split('/').pop()!, arrayBuffer);
        }
    }
    const model = await tf.loadGraphModel(new TensorFlowZipRequest(json, weights));

    return new TensorFlowModel(model);
}

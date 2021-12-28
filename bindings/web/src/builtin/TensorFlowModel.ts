import { loadTFLiteModel } from "@tensorflow/tfjs-tflite";
import * as tf from "@tensorflow/tfjs";
import { InferenceModel, Tensor } from "@tensorflow/tfjs-core";
import * as LZString from "lz-string/libs/lz-string.js";
import { Model } from "../Runtime";
import Shape from "../Shape";

// Explicitly pull in the CPU backend
import '@tensorflow/tfjs-backend-cpu';
import { toTypedArray } from "../helpers";

import { unzip } from 'unzipit';
import { TensorFlowZipRequest } from "./TensorFlowZipRequest";

export class TensorFlowModel implements Model {
    private model: InferenceModel;

    constructor(model: InferenceModel) {
        this.model = model;
    }

    static async loadTensorFlow(buffer: ArrayBuffer): Promise<TensorFlowModel> {

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

    static async loadTensorFlowLite(buffer: ArrayBuffer): Promise<TensorFlowModel> {
        const model = await loadTFLiteModel(buffer);
        return new TensorFlowModel(model);
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


async function modelToIndexedDB(model_bytes: string) {
    var data = JSON.parse(LZString.decompressFromUTF16(model_bytes)!);
    var DBOpenRequest = window.indexedDB.open("tensorflowjs", 1);
    let successes = 0;
    DBOpenRequest.onupgradeneeded = function (event) {
        const db = DBOpenRequest.result;
        var objectStore = db.createObjectStore("models_store", {
            "keyPath": "modelPath"
        });
        var objectInfoStore = db.createObjectStore("model_info_store", {
            "keyPath": "modelPath"
        });

    }
    DBOpenRequest.onsuccess = function (event) {
        const db = DBOpenRequest.result;
        data.models_store.modelArtifacts.weightData = new Uint32Array(data.weightData).buffer;
        var objectStore = db.transaction("models_store", "readwrite").objectStore("models_store");
        var objectStoreRequest = objectStore.put(data["models_store"]);
        objectStoreRequest.onsuccess = function (event) {
            successes++;
        }
        var objectInfoStore = db.transaction("model_info_store", "readwrite").objectStore("model_info_store");
        var objectInfoStoreRequest = objectInfoStore.put(data["model_info_store"]);
        objectInfoStoreRequest.onsuccess = function (event) {
            successes++;
        }
    }
    while (successes < 2) {
        await new Promise(r => setTimeout(r, 100));
    }
    return true;
}

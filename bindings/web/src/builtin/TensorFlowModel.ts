import { loadTFLiteModel } from "@tensorflow/tfjs-tflite";
import * as tf from "@tensorflow/tfjs";
import { InferenceModel, Tensor } from "@tensorflow/tfjs-core";
import * as LZString from "lz-string/libs/lz-string.js";
import { Model } from "../Runtime";
import Shape from "../Shape";

// Explicitly pull in the CPU backend
import '@tensorflow/tfjs-backend-cpu';

export class TensorFlowModel implements Model {
    private model: InferenceModel;

    constructor(model: InferenceModel) {
        this.model = model;
    }

    static async loadTensorFlow(buffer: ArrayBuffer): Promise<TensorFlowModel> {
        const decoder = new TextDecoder("utf16");
        let decoded = decodeURIComponent(escape(decoder.decode(buffer)));

        await modelToIndexedDB(decoded);
        const model_name = "imagenet_mobilenet_v3";
        const model = await tf.loadGraphModel('indexeddb://' + model_name);

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

    for (let i = 0; i <= buffers.length; i++) {
        const buffer = buffers[i];
        const shape = shapes[i];
        const arr = toTypedArray(shape.type, buffer);
        tensors.push(tf.tensor(arr, shape.values));
    }

    return tensors;
}

function toTypedArray(typeName: string, data: ArrayBuffer): any {
    switch (typeName) {
        case "f64":
            return new Float64Array(data);
        case "f32":
            return new Float32Array(data);
        case "i64":
            return new BigInt64Array(data);
        case "i32":
            return new Int32Array(data);
        case "i16":
            return new Int16Array(data);
        case "i8":
            return new Int16Array(data);
        case "u64":
            return new BigUint64Array(data);
        case "u32":
            return new Uint32Array(data);
        case "u16":
            return new Uint16Array(data);
        case "u8":
            return new Uint8Array(data);
        default:
            throw new Error(`Unknown tensor type: ${typeName}`);
    }
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

import { loadTFLiteModel, TFLiteModel } from "@tensorflow/tfjs-tflite";
import tf, { InferenceModel, Tensor, TensorLike } from "@tensorflow/tfjs";
import * as LZString from "lz-string/libs/lz-string.js";

const TensorFlowLiteMimeType = "application/tflite-model";

export const KnownCapabilities: Record<number, string> = {
    1: "rand",
    2: "sound",
    3: "accel",
    4: "image",
    5: "raw",
};
export const KnownOutputs: Record<number, string> = {
    1: "serial",
};

export const Models = {
    1: "tf.js",
    2: "tflite"
}

const ByteSize: Record<string, number> = {
    "f64": 8,
    "i64": 8,
    "u64": 8,
    "f32": 4,
    "i32": 4,
    "u32": 4,
    "u16": 2,
    "i16": 2,
    "u8": 1,
    "i8": 1
};

type CapabilityInfo = {
    capabilityType: number,
    capability: string,
    parameters: Record<string, number>,
};

type TensorDescriptor = {
    dimensions: string,
};

class Shape {
    type: string;
    values: number[];

    constructor(type: string, values: number[]) {
        this.type = type;
        this.values = values;
    }

    get tensorSize(): number {
        return this.values.reduce((product, dim) => product * dim, 1);
    }

    get byteSize(): number {
        return this.tensorSize * ByteSize[this.type];
    }
}

type ModelInfo = {
    id: number,
    modelSize: number,
    inputs?: TensorDescriptor[],
    outputs?: TensorDescriptor[],
};

type Parameters = {
    [capabilityId: number]: CapabilityInfo,
    modelid: number,
};

interface Output {
    consume(data: Uint8Array): void;
}

interface Capability {
    generate(dest: Uint8Array, id: number): void;
}

interface Imports {
    outputs: Record<number, () => Output>;
    capabilities: Record<number, () => Capability>;
}

interface Exports extends WebAssembly.Exports {
    memory: WebAssembly.Memory;
    _manifest(): void;
    _call(capability_type: number, input_type: number, capability_index: number): void;
}

interface Model {
    transform(inputArray: Uint8Array[], inputDimensions: Shape[], outputArray: Uint8Array[], outputDimensions: Shape[]): void;
}

export class Runtime {
    parameters: Parameters;
    instance: WebAssembly.Instance;

    constructor(instance: WebAssembly.Instance, params: Parameters) {
        this.instance = instance;
        this.parameters = params;
    }

    static async load(module: WebAssembly.Module, imports: Imports, modelConstructor: ModelConstructor = loadTensorflowJSModel) {
        let parameters = {
            modelid: 0,
        };
        let memory: WebAssembly.Memory;

        const { hostFunctions, finaliseModels, getParams, id } = importsToHostFunctions(
            imports,
            () => memory,
            () => parameters,
            modelConstructor,
        );
        const instance = await WebAssembly.instantiate(module, hostFunctions);

        const exports = instance.exports;
        if (!isRuneExports(exports)) {
            throw new Error("Invalid Rune exports");
        }
        memory = exports.memory;
        exports._manifest();

        // now we've asked for all the models to be loaded, let's wait until
        // they are done before continuing
        await finaliseModels();
        return new Runtime(instance, parameters);
    }

    manifest() {
        return this.exports._manifest();
    }

    call() {
        this.exports._call(0, 0, 0);
    }

    get exports() {
        // Note: checked inside Runtime.load() and exports will never change.
        const { exports } = this.instance;

        if (isRuneExports(exports)) {
            return exports;
        } else {
            throw Error();
        }
    }
}

function constructFromNameTable<T>(
    nextId: () => number,
    nameTable: Record<number, string>,
    constructors: Record<string, (id: number) => T>,
): [Record<number, T>, (type: number) => number] {
    const instances: Record<number, any> = {};

    function create(type: number) {
        const name = nameTable[type];
        if (!name) {
            throw new Error(`type ${type} is unknown`);
        }
        const constructor = constructors[name];
        if (!constructor) {
            throw new Error(`No constructor for type ${type} called \"${name}\"`);
        }

        const id = nextId();
        const instance = constructor(id);
        instances[id] = instance;
        return id;
    }
    return [instances, create];
}

type ModelConstructor = (mimetype: string, model: ArrayBuffer, p?: Parameters) => Promise<Model>;

function importsToHostFunctions(
    imports: Imports,
    getMemory: () => WebAssembly.Memory,
    getParameters: () => Parameters,
    modelConstructor: ModelConstructor,
) {
    const memory = () => {
        const m = getMemory();
        if (!m)
            throw new Error("WebAssembly memory wasn't initialized");

        return new Uint8Array(m.buffer);
    };
    const parameters = () => {
        const p = getParameters();
        if (!p)
            throw new Error("Parameters wasn't initialized");

        return p;
    };

    const ids = counter();
    const [outputs, createOutput] = constructFromNameTable(ids, KnownOutputs, imports.outputs);
    const [capabilities, createCapability] = constructFromNameTable(ids, KnownCapabilities, imports.capabilities);
    const pendingModels: Promise<[number, Model]>[] = [];
    const models: Record<number, Model> = {};
    const modelsDescription: Record<number, ModelInfo> = {};
    const utf8 = new TextDecoder();
    const decoder = new TextDecoder("utf8");

    // Annoyingly, this needs to be an object literal instead of a class.
    const env = {
        _debug(msg: number, len: number) {
            const raw = memory().subarray(msg, msg + len);
            const message = utf8.decode(raw);
            console.log(message);
        },

        request_output(type: number) {
            return createOutput(type);
        },

        consume_output(id: number, buffer: number, len: number) {
            const output = outputs[id];
            if (output) {
                const data = memory().subarray(buffer, buffer + len);
                output.consume(data);
            }
            else {
                throw new Error("Invalid output");
            }
        },

        request_capability(type: number) {
            const p = parameters();
            const id = Object.keys(p).length;
            p[id] = {
                capabilityType: type,
                capability: KnownCapabilities[type],
                parameters: {},
            };
            return createCapability(type);
        },

        request_capability_set_param(id: number,
            keyPtr: number,
            keyLength: number,
            valuePtr: number,
            valueLength: number,
            valueType: number) {
            const keyBytes = memory().subarray(keyPtr, keyPtr + keyLength);
            const key = decoder.decode(keyBytes);
            const value = memory().subarray(valuePtr, valuePtr + valueLength).slice(0);
            const valueTypeName = KnownOutputs[valueType];
            const p = parameters();
            p[id].parameters[key] = convertTypedArray(value, Int32Array);
        },

        request_provider_response(buffer: number, len: number, id: number) {
            const cap = capabilities[id];
            if (!cap) {
                throw new Error("Invalid capability");
            }
            const dest = memory().subarray(buffer, buffer + len);

            cap.generate(dest, id);
        },
        async tfm_preload_model(data: number, len: number, numInputs: number, numOutputs: number) {
            const modelData = memory().subarray(data, data + len);
            const pending = modelConstructor(TensorFlowLiteMimeType, modelData, getParameters());
            const id = ids();
            pendingModels.push(pending.then(model => [id, model]));
            modelsDescription[id] = { "id": id, "modelSize": len };
            return id;
        },
        async rune_model_load(mimetype: number, mimetype_len: number, model: number, model_len: number, input_descriptors: number, input_len: number, output_descriptors: number, output_len: number) {
            const mime = decoder.decode(memory().subarray(mimetype, mimetype + mimetype_len));
            const model_data = memory().subarray(model, model + model_len);
            //inputs
            let o = memory().subarray(input_descriptors, input_descriptors + 8 * input_len);
            let inputs = [];
            for (let i = 0; i < input_len; i++) {
                const inputs_pointer = new Uint32Array(new Uint8Array([o[i * 8], o[i * 8 + 1], o[i * 8 + 2], o[i * 8 + 3]]).buffer)[0];
                const inputs_length = new Uint32Array(new Uint8Array([o[i * 8 + 4], o[i * 8 + 5], o[i * 8 + 6], o[i * 8 + 7]]).buffer)[0];
                const inputs_string = decoder.decode(memory().subarray(inputs_pointer, inputs_pointer + inputs_length));
                inputs.push({ "dimensions": inputs_string });
            }
            //outputs
            o = memory().subarray(output_descriptors, output_descriptors + 8 * output_len);
            let outputs = [];
            for (let i = 0; i < output_len; i++) {
                const outputs_pointer = new Uint32Array(new Uint8Array([o[i * 8], o[i * 8 + 1], o[i * 8 + 2], o[i * 8 + 3]]).buffer)[0];
                const outputs_length = new Uint32Array(new Uint8Array([o[i * 8 + 4], o[i * 8 + 5], o[i * 8 + 6], o[i * 8 + 7]]).buffer)[0];
                const outputs_string = decoder.decode(memory().subarray(outputs_pointer, outputs_pointer + outputs_length));
                outputs.push({ "dimensions": outputs_string });
            }

            const pending = modelConstructor(mime, model_data);
            const id = ids();

            pendingModels.push(pending.then(model => [id, model]));
            modelsDescription[id] = { id, inputs, outputs, "modelSize": model_len };
            return id;
        },

        async rune_model_infer(id: number, inputs: number, outputs: number) {
            const model = models[id];
            let modelsDes = modelsDescription[id];

            let inputArray = [];
            let inputDimensions = [];

            for (let i = 0; i < modelsDes!.inputs!.length; i++) {
                let dimensions = parseDimensions(modelsDes!.inputs![i].dimensions);

                let o = memory().subarray(inputs + i * 4, inputs + 4 + i * 4);
                const pointer = new Uint32Array(new Uint8Array([o[0], o[1], o[2], o[3]]).buffer)[0];
                inputArray.push(memory().subarray(pointer, pointer + dimensions.byteSize));
                inputDimensions.push(dimensions);
            }

            let outputArray = [];
            let outputDimensions = [];
            for (let i = 0; i < modelsDes!.outputs!.length; i++) {
                let dimensions = parseDimensions(modelsDes!.outputs![i].dimensions);
                let o = memory().subarray(outputs + i * 4, outputs + 4 + i * 4);
                const pointer = new Uint32Array(new Uint8Array([o[0], o[1], o[2], o[3]]).buffer)[0];
                outputArray.push(memory().subarray(pointer, pointer + dimensions.byteSize));
                outputDimensions.push(dimensions);
            }
            model.transform(inputArray, inputDimensions, outputArray, outputDimensions);
            parameters().modelid++;
            return id;
        },

        tfm_model_invoke(id: number, inputPtr: number, inputLen: number, outputPtr: number, outputLen: number) {
            const model = models[id];
            if (!model) {
                throw new Error("Invalid model");
            }
            const input = memory().subarray(inputPtr, inputPtr + inputLen);
            const output = memory().subarray(outputPtr, outputPtr + outputLen);
            //for backwards compatibility with older runes using single input/output tfm_model_invoke
            transformSingleModel(model, input, output, parameters()[Object.keys(parameters()).length - 2]);
            parameters().modelid++;
            return id;
        },
    };

    async function synchroniseModelLoading() {
        const loadedModels = await Promise.all(pendingModels);
        pendingModels.length = 0;
        loadedModels.forEach(([id, model]) => {
            models[id] = model;
        });
    }
    return {
        hostFunctions: { env },
        finaliseModels: synchroniseModelLoading,
        getParams: parameters,
        id: models,
    };
}

function parseDimensions(dimensions: string): Shape {
    const pattern = /([\w\d]+)\[(\d+)(?:,\s*(\d+))\]/;
    const match = pattern.exec(dimensions);

    if (!match) {
        throw new Error();
    }

    const typeName = match.shift();

    return new Shape(typeName!, match.map(parseInt));
}

function counter() {
    let value = 0;
    return () => { value++; return value - 1; };
}

function isRuneExports(obj: any): obj is Exports {
    return (obj &&
        obj.memory instanceof WebAssembly.Memory &&
        obj._call instanceof Function &&
        obj._manifest instanceof Function);
}

class TensorFlowModel implements Model {
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
        const outputs = this.model.predict(inputs, {}) as tf.Tensor[];

        for (let i = 0; i <= outputArray.length; i++) {
            const output = outputs[i];
            const dest = outputArray[i];
            dest.set(output.dataSync());
        }
    }
}

function transformSingleModel(model: Model, input: Uint8Array, output: Uint8Array, parameters: CapabilityInfo) {
    throw new Error();
    // //for backwards compatibility with older runes using single input/output tfm_model_invoke
    // if (parameters["capability"] == "image") {
    //     var inputTyped = convertTypedArray(input, Uint8Array);

    //     //pub enum PixelFormat
    //     if (parameters["parameters"]["pixel_format"] == 2) {
    //         //GrayScale =  3
    //         input = tf.tensor2d(inputTyped, [parameters["parameters"]["width"], parameters["parameters"]["height"]]).expandDims(0);
    //     } else {
    //         //RGB = 0,
    //         //BGR = 1,
    //         //YUV = 2,
    //         input = tf.tensor3d(inputTyped, [parameters["parameters"]["width"], parameters["parameters"]["height"], 3]).expandDims(0);
    //     }
    // } else {
    //     input = convertTypedArray(input, Uint8Array);
    // }

    // const out = model.predict(input);
    // const result = out.dataSync();

    // var uint8_output = convertTypedArray(result, Uint8Array);
    // output.set(result);
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

async function loadTensorflowJSModel(mimetype: string, bytes: ArrayBuffer, parameters?: Parameters): Promise<Model> {
    // FIXME: figure out how to determine whether something is tflite or not
    const useTflite = true;

    if (useTflite) {
        return await TensorFlowModel.loadTensorFlowLite(bytes);
    } else {
        return await TensorFlowModel.loadTensorFlow(bytes);
    }
}

interface TypedArray extends ArrayBuffer {
    readonly buffer: ArrayBuffer;
}

//this function can convert any TypedArray to any other kind of TypedArray :
function convertTypedArray<T>(src: TypedArray, constructor: any): T {
    // Instantiate a buffer (zeroed out) and copy the bytes from "src" into it.
    const buffer = new constructor(src.byteLength);
    buffer.set(src.buffer);
    return buffer[0] as T;
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

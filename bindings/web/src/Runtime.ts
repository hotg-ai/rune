
import Shape from "./Shape";
import { Imports, KnownCapabilities, KnownOutputs, Model } from ".";
import { TensorFlowLiteMimeType } from "./builtin";

type CapabilityInfo = {
    capabilityType: number,
    capability: string,
    parameters: Record<string, number>,
};

type TensorDescriptor = {
    dimensions: string,
};

type ModelInfo = {
    id: number,
    modelSize: number,
    inputs?: TensorDescriptor[],
    outputs?: TensorDescriptor[],
};

type Parameters = {
    [capabilityId: number]: CapabilityInfo,
}

/**
 * Public interface exposed by the WebAssembly module.
 */
interface Exports extends WebAssembly.Exports {
    memory: WebAssembly.Memory;
    _manifest(): void;
    _call(capability_type: number, input_type: number, capability_index: number): void;
}

export class Runtime {
    parameters: Parameters;
    instance: WebAssembly.Instance;
    input: any;
    constructor(instance: WebAssembly.Instance, params: Parameters) {
        this.instance = instance;
        this.parameters = params;
    }

    static async load(wasm: ArrayBuffer, imports: Imports) {
        let parameters = {
            modelid: 0,
        };
        let memory: WebAssembly.Memory;

        const { hostFunctions, finaliseModels } = importsToHostFunctions(
            imports,
            () => memory,
            () => parameters,
        );
        const { instance } = await WebAssembly.instantiate(wasm, hostFunctions);

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

/**
 * Generate a bunch of host functions backed by the supplied @param imports.
 */
function importsToHostFunctions(
    imports: Imports,
    getMemory: () => WebAssembly.Memory,
    getParameters: () => Parameters,
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
            const p = parameters();
            p[id + 1].parameters[key] = convertTypedArray<Int32Array>(value, Int32Array)[0];
        },

        request_provider_response(buffer: number, len: number, id: number) {
            const cap = capabilities[id];
            if (!cap) {
                throw new Error("Invalid capability");
            }
            const dest = memory().subarray(buffer, buffer + len);
            cap.generate(dest, id);
        },
        tfm_preload_model(data: number, len: number, numInputs: number, numOutputs: number) {
            const modelData = memory().subarray(data, data + len);
            const handler = imports.modelHandlers[TensorFlowLiteMimeType];
            const pending = handler(modelData);
            const id = ids();
            pendingModels.push(pending.then(model => [id, model]));
            modelsDescription[id] = { "id": id, "modelSize": len };
            return id;
        },
        rune_model_load(mimetype: number, mimetype_len: number, model: number, model_len: number, input_descriptors: number, input_len: number, output_descriptors: number, output_len: number) {
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

            const handler = imports.modelHandlers[mime];
            if (!handler) {
                throw new Error(`No handler registered for model type, "${mime}"`);
            }
            const pending = handler(model_data);
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
                let dimensions = Shape.parse(modelsDes!.inputs![i].dimensions);

                let o = memory().subarray(inputs + i * 4, inputs + 4 + i * 4);
                const pointer = new Uint32Array(new Uint8Array([o[0], o[1], o[2], o[3]]).buffer)[0];
                inputArray.push(memory().subarray(pointer, pointer + dimensions.byteSize));
                inputDimensions.push(dimensions);
            }

            let outputArray = [];
            let outputDimensions = [];
            for (let i = 0; i < modelsDes!.outputs!.length; i++) {
                let dimensions = Shape.parse(modelsDes!.outputs![i].dimensions);
                let o = memory().subarray(outputs + i * 4, outputs + 4 + i * 4);
                const pointer = new Uint32Array(new Uint8Array([o[0], o[1], o[2], o[3]]).buffer)[0];
                outputArray.push(memory().subarray(pointer, pointer + dimensions.byteSize));
                outputDimensions.push(dimensions);
            }
            model.transform(inputArray, inputDimensions, outputArray, outputDimensions);
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
    };
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


function transformSingleModel(model: Model, input: Uint8Array, output: Uint8Array, parameters: CapabilityInfo) {
    throw new Error(`Unable to do inference with this model. Please rebuild the Rune with "rune v0.5"`);
}

interface TypedArray extends ArrayBuffer {
    readonly buffer: ArrayBuffer;
}

//this function can convert any TypedArray to any other kind of TypedArray :
function convertTypedArray<T>(src: TypedArray, constructor: any): T {
    // Instantiate a buffer (zeroed out) and copy the bytes from "src" into it.
    return new constructor(src.buffer) as T;
}
import Shape from "./Shape";
import { Capability, Imports, Model, Output } from ".";
import { TensorFlowLiteMimeType } from "./builtin";
import { IdentityInputs } from "@tensorflow/tfjs-core";

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

/**
 * Public interface exposed by the WebAssembly module.
 */
interface Exports extends WebAssembly.Exports {
    memory: WebAssembly.Memory;
    _manifest(): void;
    _call(capability_type: number, input_type: number, capability_index: number): void;
}

export class Runtime {
    instance: WebAssembly.Instance;

    constructor(instance: WebAssembly.Instance) {
        this.instance = instance;
    }

    static async load(wasm: ArrayBuffer, imports: Imports) {
        let memory: WebAssembly.Memory;

        const { hostFunctions, finaliseModels } = importsToHostFunctions(
            imports,
            () => memory,
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
        return new Runtime(instance);
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

type Dict<Key extends keyof any, Value> = Partial<Record<Key, Value>>;

/**
 * Generate a bunch of host functions backed by the supplied @param imports.
 */
function importsToHostFunctions(
    imports: Imports,
    getMemory: () => WebAssembly.Memory,
) {
    const memory = () => {
        const m = getMemory();
        if (!m)
            throw new Error("WebAssembly memory wasn't initialized");

        return new Uint8Array(m.buffer);
    };

    const ids = counter();
    const outputs: Dict<number, Output> = {};
    const capabilities: Dict<number, Capability> = {};
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
            const output = imports.createOutput(type);
            const id = ids();

            outputs[id] = output;
            return id;
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
            const capability = imports.createCapability(type);
            const id = ids();

            capabilities[id] = capability;
            return id;
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

            const capability = capabilities[id];

            if (!capability) {
                throw new Error(`Tried to set "${key}" to ${value} but capability ${id} doesn't exist`);
            }

            // TODO: use valueType to figure out what type of array to convert to
            // instead of assuming Int32Array.
            capability.setParameter(key, convertTypedArray<Int32Array>(value, Int32Array)[0]);
        },

        request_provider_response(buffer: number, len: number, id: number) {
            const cap = capabilities[id];
            if (!cap) {
                throw new Error("Invalid capability");
            }
            const dest = memory().subarray(buffer, buffer + len);

            cap.generate(dest);
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

            const pending = imports.createModel(mime, model_data);
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
            deprecated("tfm_model_invoke()", "0.5");
        },
        tfm_preload_model(data: number, len: number, numInputs: number, numOutputs: number) {
            deprecated("tfm_preload_model()", "0.5");
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


interface TypedArray extends ArrayBuffer {
    readonly buffer: ArrayBuffer;
}

//this function can convert any TypedArray to any other kind of TypedArray :
function convertTypedArray<T>(src: TypedArray, constructor: any): T {
    return new constructor(src.buffer) as T;
}


function deprecated(feature: string, version: string) {
    throw new Error(`This runtime no longer supports Runes using "${feature}". Please rebuild with Rune ${version}`);
}

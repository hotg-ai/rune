import { Imports, KnownCapabilities, KnownOutputs, Model, ModelConstructor } from "./imports";

export default class Runtime {
    private instance: WebAssembly.Instance;

    private constructor(instance: WebAssembly.Instance) {
        this.instance = instance;
    }

    public static async load(mod: WebAssembly.Module, imports: Imports, modelConstructor: ModelConstructor): Promise<Runtime> {
        let memory: WebAssembly.Memory;

        const instance = await WebAssembly.instantiate(mod, importsToHostFunctions(imports, () => memory, modelConstructor));

        if (!isRuneExports(instance.exports)) {
            throw new Error("Invalid Rune exports");
        }
        memory = instance.exports.memory;

        instance.exports._manifest();

        return new Runtime(instance);
    }

    public call() {
        this.exports._call(0, 0, 0);
    }

    private get exports(): RuneExports {
        if (!isRuneExports(this.instance.exports)) {
            throw new Error("Invalid Rune exports");
        }

        return this.instance.exports;
    }
}

type HostFunctions = {
    env: {
        _debug(msg: number, len: number): void;
        request_output(type: number): number;
        consume_output(id: number, buffer: number, len: number): void;
        request_capability(type: number): number;
        request_capability_set_param(): void;
        request_provider_response(buffer: number, len: number, id: number): void;
        tfm_preload_model(data: number, len: number, _: number): number;
        tfm_model_invoke(id: number, inputPtr: number, inputLen: number, outputPtr: number, outputLen: number): void;
    }
}

function importsToHostFunctions(imports: Imports,
    getMemory: () => WebAssembly.Memory,
    modelConstructor: ModelConstructor): HostFunctions {
    const memory = () => {
        const m = getMemory();
        if (!m) throw new Error("WebAssembly memory wasn't initialized");
        return new Uint8Array(m.buffer);
    };
    const ids = counter();
    const numberedOutputs = new IndirectFactory(ids, KnownOutputs, imports.outputs);
    const numberedCapability = new IndirectFactory(ids, KnownCapabilities, imports.capabilities);
    const models = new Map<number, Model>();
    const utf8 = new TextDecoder();

    // Annoyingly, this needs to be an object literal instead of a class.
    const env = {
        _debug(msg: number, len: number) {
            const raw = memory().subarray(msg, msg + len);
            const message = utf8.decode(raw);
            console.log(message);
        },
        request_output(type: number) {
            const [id, _] = numberedOutputs.create(type);
            return id;
        },
        consume_output(id: number, buffer: number, len: number) {
            const output = numberedOutputs.instances.get(id);

            if (output) {
                const data = memory().subarray(buffer, buffer + len);
                output.consume(data);
            } else {
                throw new Error("Invalid output");
            }
        },
        request_capability(type: number) {
            const [id, _] = numberedCapability.create(type);
            return id;
        },
        request_capability_set_param() { console.error("request_capability_set_param", arguments); },
        request_provider_response(buffer: number, len: number, id: number) {
            const cap = numberedCapability.instances.get(id);
            if (!cap) {
                throw new Error("Invalid capabiltiy");
            }

            const dest = memory().subarray(buffer, buffer + len);
            cap.generate(dest);
        },
        tfm_preload_model(data: number, len: number, _: number) {
            const modelData = memory().subarray(data, data + len);
            const model = modelConstructor(modelData);
            const id = ids();
            models.set(id, model);
            return id;
        },
        tfm_model_invoke(id: number, inputPtr: number, inputLen: number, outputPtr: number, outputLen: number) {
            const model = models.get(id);

            if (!model) {
                throw new Error("Invalid model");
            }

            const input = memory().subarray(inputPtr, inputPtr + inputLen);
            const output = memory().subarray(outputPtr, outputPtr + outputLen);

            model.transform(input, output);
        },
    };

    return { env };
}

function counter(): () => number {
    let value = 0;

    return () => value++;
}

class IndirectFactory<T> {
    nextId: () => number;
    nameTable: Record<number, string>;
    constructors: Record<string, () => T>;
    instances: Map<number, T>;

    constructor(nextId: () => number, nameTable: Record<number, string>, constructors: Record<string, () => T>) {
        this.nextId = nextId;
        this.nameTable = nameTable;
        this.constructors = constructors;
        this.instances = new Map<number, T>();
    }

    create(type: number): [number, T] {
        const name = this.nameTable[type];
        if (!name) {
            throw new Error(`type ${type} is unknown`);
        }
        const constructor = this.constructors[name];
        if (!constructor) {
            throw new Error(`No constructor for type ${type} called \"${name}\"`);
        }

        const instance = constructor();
        const id = this.nextId();

        this.instances.set(id, instance);
        return [id, instance];
    }
}

interface RuneExports {
    readonly memory: WebAssembly.Memory;
    _call(_a: number, _b: number, _c: number): void;
    _manifest(): void;
}

function isRuneExports(obj?: any): obj is RuneExports {
    return obj &&
        obj.memory instanceof WebAssembly.Memory &&
        obj._call instanceof Function &&
        obj._manifest instanceof Function;
}

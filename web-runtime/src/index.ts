const KnownCapabilities = {
    1: "rand",
    2: "sound",
    3: "accel",
    4: "image",
    5: "raw",
} as const;

const KnownOutputs = {
    1: "serial",
} as const;

export interface Capability {
    generate(dest: Uint8Array): void;
    set(key: string, value: number): void;
}

export interface Capabilities {
    [key: string]: () => Capability;
};

export interface Output {
    consume(data: Uint8Array): void;
}

export interface Outputs {
    [key: string]: () => Output;
};

export interface Model {
    transform(input: Uint8Array, output: Uint8Array): void;
}

export interface Imports {
    capabilities: Capabilities;
    outputs: Outputs;
    loadModel(raw: Uint8Array): Model;
}

export type Runtime = () => void;

export async function loadRuntime(mod: WebAssembly.Module, imports: Imports): Promise<Runtime> {
    let memory: WebAssembly.Memory;

    const instance = await WebAssembly.instantiate(mod, makeImports(imports, () => memory));

    const { memory: mem, _manifest } = instance.exports;

    if (mem instanceof WebAssembly.Memory) {
        memory = mem;
    } else {
        throw new Error("The Rune should export its memory as \"memory\"");
    }

    if (_manifest instanceof Function) {
        _manifest();
    } else {
        throw new Error("The Rune should export a \"_manifest()\" function");
    }

    return () => {
        const { _call } = instance.exports;

        if (_call instanceof Function) {
            _call(0, 0, 0);
        } else {
            throw new Error("The Rune should export a \"_call()\" function");
        }
    };
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

function makeImports(imports: Imports, getMemory: () => WebAssembly.Memory): WebAssembly.Imports {
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
                console.log(id, buffer, len, data);
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
            const model = imports.loadModel(modelData);
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

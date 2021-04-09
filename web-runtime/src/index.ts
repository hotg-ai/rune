const KnownCapabilities = {
    rand: 1,
    sound: 2,
    accel: 3,
    image: 4,
    raw: 5,
} as const;

const KnownOutputs = {
    serial: 1,
} as const;

export interface Capability {
    [name: string]: number;
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

export interface Imports {
    capabilities(): Capabilities;
    outputs(): Outputs;
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

function makeImports(imports: Imports, getMemory: () => WebAssembly.Memory): WebAssembly.Imports {
    const memory = () => {
        const m = getMemory();
        if (!m) {
            throw new Error("WebAssembly memory wasn't initialized");
        }

        return new Uint8Array(m.buffer);
    };
    const utf8 = new TextDecoder();

    // Annoyingly, this needs to be an object literal instead of a class.
    const env = {
        _debug(msg: number, len: number) {
            const raw = memory().subarray(msg, msg + len);
            const message = utf8.decode(raw);
            console.log(message);
        },
        request_output(type: number) { console.error("request_output"); },
        consume_output() { console.error("consume_output"); },
        request_capability() { console.error("request_capability"); },
        request_capability_set_param() { console.error("request_capability_set_param"); },
        request_provider_response() { console.error("request_provider_response"); },
        tfm_preload_model() { console.error("tfm_preload_model"); },
        tfm_model_invoke() { console.error("tfm_model_invoke"); },
    };

    return { env };
}

export const KnownCapabilities = {
    1: "rand",
    2: "sound",
    3: "accel",
    4: "image",
    5: "raw",
} as const;

export const KnownOutputs = {
    1: "serial",
} as const;

export interface Capability {
    generate(dest: Uint8Array): void;
    set(key: string, value: number): void;
}

export interface Capabilities {
    [key: string]: () => Capability;
}

export interface Output {
    consume(data: Uint8Array): void;
}

export interface Outputs {
    [key: string]: () => Output;
}

export interface Model {
    transform(input: Uint8Array, output: Uint8Array): void;
}

export type ModelConstructor = (raw: Uint8Array) => Promise<Model>;

export interface Imports {
    capabilities: Capabilities;
    outputs: Outputs;
}

export type HostFunctions = {
    env: {
        _debug(msg: number, len: number): void;
        request_output(type: number): number;
        consume_output(id: number, buffer: number, len: number): void;
        request_capability(type: number): number;
        request_capability_set_param(): void;
        request_provider_response(buffer: number, len: number, id: number): void;
        tfm_preload_model(data: number, len: number, _: number): number;
        tfm_model_invoke(
            id: number,
            inputPtr: number,
            inputLen: number,
            outputPtr: number,
            outputLen: number
        ): void;
    };
};

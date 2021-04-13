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

export type ModelConstructor = (raw: Uint8Array) => Model;

export interface Imports {
    capabilities: Capabilities;
    outputs: Outputs;
}

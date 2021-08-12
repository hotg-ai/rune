export { Runtime } from "./Runtime";
import Shape from "./Shape";

export const TensorFlowLiteMimeType = "application/tflite-model";

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

export interface Output {
    consume(data: Uint8Array): void;
}

export interface Capability {
    generate(dest: Uint8Array, id: number): void;
}

export interface Imports {
    outputs: Record<number, () => Output>;
    capabilities: Record<number, (capabilityType: number) => Capability>;
}

export interface Model {
    transform(inputArray: Uint8Array[], inputDimensions: Shape[], outputArray: Uint8Array[], outputDimensions: Shape[]): void;
}

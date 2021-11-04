import { Builder } from "./facade";
import { TensorFlowModel, TensorFlowLiteMimeType } from "./builtin";

export const Capabilities = {
    "rand": 1,
    "sound": 2,
    "accel": 3,
    "image": 4,
    "raw": 5,
} as const;

export type CapabilityType = keyof typeof Capabilities;

/**
 * Use a high level builder API to initialize the Rune runtime.
 *
 * Check out the "Runtime" module if you need tighter control over the runtime
 * or want to avoidunnecessary indirection/copies.
 */
export function builder(): Builder {
    return new Builder()
        .withModelHandler(TensorFlowLiteMimeType, TensorFlowModel.loadTensorFlowLite);
}

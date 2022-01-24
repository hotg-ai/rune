export { InputDescription, OutputValue, ReadInput, Result, Builder, Evaluate } from "./facade";
export { default as Shape } from "./Shape";
export { default as Tensor } from "./Tensor";

import { Builder } from "./facade";

/**
 * A map of capability names to their identifies.
 */
export const Capabilities = {
    "rand": 1,
    "sound": 2,
    "accel": 3,
    "image": 4,
    "raw": 5,
} as const;

/**
 * A map of output names to their identifies.
 */
export const Outputs = {
    "serial": 1,
} as const;

/**
 * The name of all known capabilities.
 */
export type CapabilityType = keyof typeof Capabilities;

/**
 * The name of all known outputs.
 */
export type OutputType = keyof typeof Outputs;

/**
 * Use a high level builder API to initialize the Rune runtime.
 *
 * Check out the "Runtime" module if you need tighter control over the runtime
 * or want to avoid unnecessary indirection/copies.
 */
export function builder(): Builder {
    return new Builder();
}

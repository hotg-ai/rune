export { consoleLogger } from "./logging";
export type { Logger } from "./logging";
export { RuneLoader } from "./RuneLoader";

import { runtime_v1 } from "@hotg-ai/rune-wit-files";
import { TensorDescriptor, Tensors } from "./proc_blocks";

export type Tensor = runtime_v1.Tensor;
export const ElementType = runtime_v1.ElementType;
export type ElementType = runtime_v1.ElementType;
export type Dimensions = runtime_v1.Dimensions;

/**
 * A callback that can load models.
 */
export type ModelHandler = (
  model: ArrayBuffer,
  args: Record<string, string>
) => Promise<Node>;

export interface Node {
  graph(args: Record<string, string>): Promise<Tensors>;
  infer(
    inputs: Record<string, runtime_v1.Tensor>,
    args: Record<string, string>
  ): Promise<Record<string, runtime_v1.Tensor>>;
}

export interface Runtime {
  /**
   * Run the entire Rune pipeline.
   */
  infer(): Promise<void>;
  /**
   * Get all named inputs.
   */
  inputs: Record<string, TensorDescriptor>;
  /**
   * Set an input tensor by name.
   */
  setInput(name: string, tensor: Tensor): void;
}

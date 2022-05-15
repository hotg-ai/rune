import { runtime_v1 } from "@hotg-ai/rune-wit-files";
import type { Tensor } from "..";
import type { TensorDescriptor, Tensors } from "../proc_blocks";

export { RuneLoader } from "./RuneLoader";

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
    args: Record<string, string>,
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
  inputs(): Record<string, TensorDescriptor>;
  /**
   * Set an input tensor by name.
   */
  setInput(name: string, tensor: Tensor): void;
}

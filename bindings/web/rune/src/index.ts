import { runtime_v1 } from "@hotg-ai/rune-wit-files";

export { consoleLogger } from "./logging";
export type { Logger } from "./logging";
export { RuneLoader } from "./RuneLoader";
export type { Runtime } from "./RuneLoader";
import { Tensors } from "./proc_blocks";

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

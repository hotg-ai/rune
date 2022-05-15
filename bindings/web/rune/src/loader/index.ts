import { runtime_v1 } from "@hotg-ai/rune-wit-files";

export { RuneLoader } from "./RuneLoader";

export interface Model {}

/**
 * A callback that can load models.
 */
export type ModelHandler = (
  model: ArrayBuffer,
  args: Record<string, string>
) => Promise<Model>;

export interface Node {
  infer(
    inputs: Record<string, runtime_v1.Tensor>
  ): Promise<Record<string, runtime_v1.Tensor>>;
}

export type Pipeline = {
  graph: Record<number, Node>;
};

export interface Runtime {
  infer(): Promise<void>;
}

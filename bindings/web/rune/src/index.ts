import { runtime_v1 } from "@hotg-ai/rune-wit-files";
import { Logger } from "pino";
import { TensorDescriptor, Tensors } from "./proc_blocks";

export { Rune } from "./Rune";
export * from "./proc_blocks";

export type Tensor = runtime_v1.Tensor;
export const ElementType = runtime_v1.ElementType;
export type ElementType = runtime_v1.ElementType;
export type Dimensions = runtime_v1.Dimensions;

/**
 * A callback that can be used to load models.
 *
 * The callback is given the model's bytes, arguments that were associated with
 * this particular node, and a logger that should be used for logging.
 */
export type ModelHandler = (
  model: ArrayBuffer,
  args: Record<string, string>,
  logger: Logger
) => Promise<Node>;

/**
 * A node in the Rune pipeline.
 */
export interface Node {
  /**
   * Given the provided set of arguments, what are do node's input and output
   * tensors look like?
   */
  graph(args: Record<string, string>): Promise<Tensors>;

  /**
   * Evaluate this node.
   *
   * @param inputs The node's input tensors.
   * @param args Arguments that may alter this node's behaviour.
   */
  infer(
    inputs: Record<string, runtime_v1.Tensor>,
    args: Record<string, string>
  ): Promise<Record<string, runtime_v1.Tensor>>;
}

type NamedTensor = { readonly id: number } & Readonly<TensorDescriptor>;

type NodeInfo = {
  readonly inputs: readonly NamedTensor[];
  readonly outputs: readonly NamedTensor[];
  readonly args: Readonly<Record<string, string>>;
};

type Pipeline = {
  readonly inputNodes: readonly string[];
  readonly outputNodes: readonly string[];
  readonly nodes: Readonly<Record<string, NodeInfo>>;
};

/**
 * An instantiated Rune.
 */
export interface Runtime {
  readonly pipeline: Pipeline;

  /**
   * Run the Rune's pipeline.
   */
  infer(): Promise<void>;

  /**
   * Set the tensor to be used for a particular node's input by index.
   */
  setInputTensor(node: string, index: number, tensor: Tensor): void;
  /**
   * Set the tensor to be used for a particular node's input by name.
   */
  setInputTensor(node: string, name: string, tensor: Tensor): void;

  /**
   * Get a node's output tensor by index.
  */
  getOutputTensor(node: string, index: number): Tensor|undefined;
  /**
   * Get a node's output tensor by name.
  */
  getOutputTensor(node: string, name: string): Tensor|undefined;
}

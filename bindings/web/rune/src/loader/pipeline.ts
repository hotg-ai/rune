import type { Node } from ".";
import { Logger, StructuredLogger } from "../logging";
import { TensorDescriptor } from "../proc_blocks";
import { DocumentV1 } from "../Runefile";

type NodeId = number;
type TensorId = number;

export type Pipeline = {
  nodes: Record<NodeId, Node>;
  nodeInfo: Record<NodeId, NodeInfo>;
  evaluationOrder: NodeId[];
  inputs: NodeId[];
  tensors: Record<TensorId, TensorDescriptor>;
};

type NodeInfo = {
  name: string;
  args: Record<string, string>;
  inputs: Record<string, TensorId>;
  outputs: Record<string, TensorId>;
};

export function determinePipeline(doc: DocumentV1, logBackend: Logger): Pipeline {
  const logger = new StructuredLogger(logBackend, "determinePipeline");

  throw new Error();
}

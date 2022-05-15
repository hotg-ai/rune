import type { Node } from ".";
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

export function determinePipeline(doc: DocumentV1): Pipeline {
  throw new Error();
}

import { runtime_v1 } from "@hotg-ai/rune-wit-files";
import type { Node } from ".";
import { Dimensions, ElementType } from "..";
import { Logger, StructuredLogger } from "../logging";
import { TensorDescriptor, Tensors } from "../proc_blocks";
import { CapabilityStage, DocumentV1, Input, Stage } from "../Runefile";

type NodeId = string;
type TensorId = number;
type PortId = [NodeId, number];

export type TensorShape = {
  elementType: runtime_v1.ElementType;
  dimensions: Dimensions;
};

export type Pipeline = {
  nodes: Record<NodeId, Node>;
  nodeInfo: Record<NodeId, NodeInfo>;
  evaluationOrder: NodeId[];
  inputs: NodeId[];
  tensors: Record<TensorId, TensorShape>;
  outputTensors: TensorId[];
};

type NodeInfo = {
  readonly name: string;
  readonly args: Readonly<Record<string, string>>;
  readonly inputs: Record<string, TensorId>;
  readonly outputs: Record<string, TensorId>;
};

type Edge = {
  previous: PortId;
  next: PortId;
};

interface ProcBlockLike {
  graph(args: Record<string, string>): Tensors;
  evaluate(
    inputs: Record<string, runtime_v1.Tensor>,
    args: Record<string, string>
  ): Record<string, runtime_v1.Tensor>;
}

export async function determinePipeline(
  doc: DocumentV1,
  procBlocks: Record<string, ProcBlockLike>,
  models: Record<string, Node>,
  logBackend: Logger
): Promise<Pipeline> {
  const logger = new StructuredLogger(logBackend, "determinePipeline");
  logger.debug("Deriving the pipeline");

  const nodePorts = {
    ...(await ports(doc, models)),
    ...(await ports(doc, procBlocks)),
    ...inputPorts(doc),
  };

  const tensors = discoverTensors(nodePorts);
  const edges = discoverEdges(doc);

  console.log(JSON.stringify({nodePorts, tensors, edges}, null, 2));

  return {
    evaluationOrder: [],
    inputs: [],
    nodeInfo: {},
    nodes: {},
    tensors: {},
    outputTensors: [],
  };
}

function inputPorts(doc: DocumentV1): Record<string, Tensors> {
  const ports: Record<string, Tensors> = {};

  for (const [name, stage] of Object.entries(doc.pipeline)) {
    if (!isInputStage(stage)) {
      continue;
    }

    const outputs = stage.outputs?.map(({ type, dimensions }, i) => {
      const dims: Dimensions = dimensions
        ? { tag: "fixed", val: Uint32Array.from(dimensions) }
        : { tag: "dynamic" };

      const elementType = elementTypeFromName(type);
      return {
        name: i.toString(),
        elementType,
        dimensions: dims,
      };
    });
    ports[name] = { inputs: [], outputs: outputs! };
  }

  return ports;
}

const elementNames: Partial<Record<string, ElementType>> = {
  u8: runtime_v1.ElementType.U8,
  i8: runtime_v1.ElementType.I8,
  u16: runtime_v1.ElementType.U16,
  i16: runtime_v1.ElementType.I16,
  u32: runtime_v1.ElementType.U32,
  i32: runtime_v1.ElementType.I32,
  f32: runtime_v1.ElementType.F32,
  u64: runtime_v1.ElementType.U64,
  i64: runtime_v1.ElementType.I64,
  f64: runtime_v1.ElementType.F64,
  utf8: runtime_v1.ElementType.Utf8,
};

function elementTypeFromName(name: string): ElementType {
  name = name.toLowerCase();
  const type = elementNames[name];

  if (!type) {
    throw new Error(`Unknown element type, "${name}"`);
  }

  return type;
}

async function ports(
  doc: DocumentV1,
  nodes: Record<
    string,
    { graph(args: Record<string, string>): Tensors | Promise<Tensors> }
  >
): Promise<Record<NodeId, Tensors>> {
  const ports: Record<string, Tensors> = {};

  for (const [name, node] of Object.entries(nodes)) {
    const args = stageArguments(doc.pipeline[name]);
    ports[name] = await node.graph(args);
  }

  return ports;
}

function stageArguments({ args }: Stage): Record<string, string> {
  if (!args) {
    return {};
  }

  const stringified: Record<string, string> = {};

  for (const [key, value] of Object.entries(args)) {
    stringified[key] = value.toString();
  }

  return stringified;
}

function isInputStage(stage: Stage): stage is CapabilityStage {
  return "capability" in stage;
}

function discoverEdges(doc: DocumentV1): Edge[] {
  const edges: Edge[] = [];

  for (const [name, stage] of Object.entries(doc.pipeline)) {
    if (isInputStage(stage) || !stage.inputs) {
      continue;
    }

    stage.inputs.forEach((input, ix) => {
      const previous = parsePortId(input);
      edges.push({ previous, next: [name, ix] });
    });
  }

  return edges;
}

function parsePortId(value: string): PortId {
  const match = value.match(/^[\w\d_-]+(?:\.(\d+))?$/);

  if (!match) {
    throw new Error(`Unable to parse the input, "${value}"`);
  }

  const name = match[0];
  const index = match[1] ? parseInt(match[1]) : 0;

  return [name, index];
}

function discoverTensors(
  nodePorts: Record<string, Tensors>
): Array<{ port: PortId; descriptor: TensorDescriptor }> {
  return Object.entries(nodePorts).flatMap(([name, tensors]) =>
    tensors.inputs.map((tensor, ix) => ({
      port: [name, ix],
      descriptor: tensor,
    }))
  );
}

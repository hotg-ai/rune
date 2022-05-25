import { runtime_v1 } from "@hotg-ai/rune-wit-files";
import type { Node } from ".";
import { Dimensions, ElementType } from "..";
import {
  isCapabilityStage,
  isModelStage,
  isOutStage,
  isProcBlockStage,
} from "../utils";
import { Logger, StructuredLogger } from "../logging";
import { TensorDescriptor, Tensors } from "../proc_blocks";
import {
  CapabilityStage,
  DocumentV1,
  Input,
  ModelStage,
  OutStage,
  ProcBlockStage,
  Stage,
} from "../Runefile";

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
  outputTensors: Record<NodeId, TensorId[]>;
};

type NodeInfo = {
  readonly name: string;
  readonly args: Readonly<Record<string, string>>;
  readonly inputs: Record<string, TensorId>;
  readonly outputs: Record<string, TensorId>;
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

  const resolver = new PipelineResolver(doc, procBlocks, models);
  return await resolver.pipeline();
}

type TensorInfo = {
  parent: NodeId;
  index: number;
  shape: TensorDescriptor;
  isGlobalInput: boolean;
};

class PipelineResolver {
  inputNodes: NodeId[] = [];
  inputsAndOutputs: Record<NodeId, Tensors> = {};
  tensors: TensorInfo[] = [];
  outputNodes: NodeId[] = [];
  tensorInputs: Record<NodeId, Record<string, TensorId>> = {};
  outputTensors: Record<NodeId, TensorId[]> = {};

  constructor(
    private doc: DocumentV1,
    private procBlocks: Record<string, ProcBlockLike>,
    private models: Record<string, Node>
  ) {}

  async pipeline(): Promise<Pipeline> {
    await this.registerStages();
    this.registerTensors();
    this.resolveInputs();
    this.resolveOutputTensors();

    return {
      evaluationOrder: ["rand", "mod360", "sine"],
      inputs: this.inputNodes,
      nodeInfo: this.nodeInfo(),
      nodes: this.nodes(),
      outputTensors: this.outputTensors,
      tensors: this.tensorShapes(),
    };
  }

  resolveOutputTensors() {
    for (const node of this.outputNodes) {
      const inputNames = stageInputs(this.doc.pipeline[node]);

      const inputs: TensorId[] = inputNames
        .map(parsePortId)
        .map(([upstreamNode, outputIndex]) => {
          const ix = this.tensors.findIndex((t) => {
            return t.parent == upstreamNode && t.index == outputIndex;
          });
          if (typeof ix != "number") {
            throw new Error(
              `Unable to find the "${upstreamNode}.${outputIndex}" input used by "${node}"`
            );
          }

          return ix;
        });

      this.outputTensors[node] = inputs;
    }
  }

  nodes(): Record<string, Node> {
    const nodes = { ...this.models };

    for (const [name, procBlock] of Object.entries(this.procBlocks)) {
      nodes[name] = {
        graph: (args) => Promise.resolve(procBlock.graph(args)),
        infer: (inputs, args) =>
          Promise.resolve(procBlock.evaluate(inputs, args)),
      };
    }

    return nodes;
  }

  tensorShapes(): Record<TensorId, TensorShape> {
    const entries = this.tensors.map((t, id) => [id, t.shape]);
    return Object.fromEntries(entries);
  }

  nodeInfo(): Record<string, NodeInfo> {
    const nodes: Record<string, NodeInfo> = {};

    for (const [name, stage] of Object.entries(this.doc.pipeline)) {
      if (isOutStage(stage)) {
        // TODO: handle output nodes
        continue;
      }

      const args = stageArguments(stage);
      const inputs = this.tensorInputs[name];
      nodes[name] = { name, args, inputs, outputs: {} };
    }

    return nodes;
  }

  async registerStages() {
    for (const [name, stage] of Object.entries(this.doc.pipeline)) {
      const args = stageArguments(stage);

      if (isCapabilityStage(stage)) {
        this.inputNodes.push(name);
        const graph = this.graph(name, args);
        console.log("[Capability]", name, graph);
        this.inputsAndOutputs[name] = graph;
      } else if (isProcBlockStage(stage)) {
        this.inputsAndOutputs[name] = this.graph(name, args);
      } else if (isModelStage(stage)) {
        this.inputsAndOutputs[name] = await this.modelGraph(name, args);
      } else {
        this.outputNodes.push(name);
      }
    }
  }

  graph(node: string, args: Record<string, string>): Tensors {
    if (node in this.procBlocks) {
      return this.procBlocks[node].graph(args);
    } else {
      throw new Error(`No "${node}" proc-block registered`);
    }
  }

  async modelGraph(
    node: string,
    args: Record<string, string>
  ): Promise<Tensors> {
    if (node in this.models) {
      return await this.models[node].graph(args);
    } else {
      throw new Error(`No "${node}" model registered`);
    }
  }

  registerTensors() {
    for (const node in this.inputsAndOutputs) {
      const { outputs } = this.inputsAndOutputs[node];

      if (node in this.inputNodes) {
        // Input nodes aren't connected to anything, so we need to allocate
        // their input tensors explicitly.
        const { inputs } = this.inputsAndOutputs[node];
        if (inputs.length != 1) {
          throw new Error();
        }
        this.tensors.push({
          parent: node,
          index: 0,
          shape: inputs[0],
          isGlobalInput: true,
        });
      }

      outputs.forEach((shape, index) => {
        this.tensors.push({ parent: node, index, shape, isGlobalInput: false });
      });
    }
  }

  /**
   * For each stage, find the TensorId of its inputs and map them to the name
   * used by the stage.
   */
  resolveInputs() {
    for (const [node, stage] of Object.entries(this.doc.pipeline)) {
      if (isOutStage(stage)) {
        // Outputs are handed separately.
        continue;
      }

      if (isCapabilityStage(stage)) {
        const ix = this.tensors.findIndex(
          (t) => t.isGlobalInput && t.parent == node
        );
        this.tensorInputs[node] = { [node]: ix! };
        continue;
      }

      const inputs: TensorId[] = stageInputs(stage)
        .map(parsePortId)
        .map(([upstreamNode, outputIndex]) => {
          const ix = this.tensors.findIndex((t) => {
            return (
              t.parent == upstreamNode &&
              t.index == outputIndex &&
              !t.isGlobalInput
            );
          });
          if (typeof ix != "number") {
            throw new Error(
              `Unable to find the "${upstreamNode}.${outputIndex}" input used by "${node}"`
            );
          }

          return ix;
        });

      const namedInputs: Record<string, TensorId> = {};

      for (let i = 0; i < inputs.length; i++) {
        // Note: we assume the proc-block declared in the same order as they
        // are used in the Runefile
        const tensorId = inputs[i];
        const { name } = this.inputsAndOutputs[node].inputs[i];
        namedInputs[name] = tensorId;
      }

      this.tensorInputs[node] = namedInputs;
    }
  }
}

function stageInputs(stage: Stage): string[] {
  if (
    (isProcBlockStage(stage) || isModelStage(stage) || isOutStage(stage)) &&
    stage.inputs
  ) {
    return stage.inputs;
  }

  return [];
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

function parsePortId(value: string): PortId {
  const match = value.match(/^[\w\d_-]+(?:\.(\d+))?$/);

  if (!match) {
    throw new Error(`Unable to parse the input, "${value}"`);
  }

  const name = match[0];
  const index = match[1] ? parseInt(match[1]) : 0;

  return [name, index];
}

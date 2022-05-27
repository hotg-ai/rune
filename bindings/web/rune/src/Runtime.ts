import { Node } from ".";
import { runtime_v1 } from "@hotg-ai/rune-wit-files";
import { TensorDescriptor, Tensors } from "./proc_blocks";
import { DocumentV1, Stage } from "./Runefile";
import { Tensor } from ".";
import {
  isCapabilityStage,
  isOutStage,
  stageArguments,
  stageInputs,
} from "./utils";
import { Logger } from "pino";

type TensorId = number;
type NodeId = string;

interface ProcBlockLike {
  graph(args: Record<string, string>): Tensors;
  evaluate(
    inputs: Record<string, runtime_v1.Tensor>,
    args: Record<string, string>
  ): Record<string, runtime_v1.Tensor>;
}

export async function create(
  doc: DocumentV1,
  procBlocks: Record<string, ProcBlockLike>,
  models: Record<string, Node>,
  logger: Logger
): Promise<Runtime> {
  const pb = procBlockNodes(procBlocks);
  const nodes: Record<string, Node> = { ...models, ...pb };

  const { dependencies, evaluationOrder } = await getTensors(
    doc.pipeline,
    nodes
  );
  return new Runtime(doc, nodes, dependencies, evaluationOrder, logger);
}

type NodeDependencies = {
  inputs: Record<string, TensorId>;
  outputs: Record<string, TensorId>;
};

type Stuff = {
  evaluationOrder: NodeId[];
  dependencies: Record<NodeId, NodeDependencies>;
};

function count(): () => number {
  let n = 0;
  return () => n++;
}

async function allGraphs(
  pipeline: Record<NodeId, Stage>,
  nodes: Record<NodeId, Node>
): Promise<Record<NodeId, Tensors>> {
  const promises = Object.entries(pipeline).map(async ([name, stage]) => {
    const args = stageArguments(stage);
    const node = nodes[name];
    const tensors = await node.graph(args);
    return [name, tensors] as const;
  });

  return Object.fromEntries(await Promise.all(promises));
}

async function getTensors(
  pipeline: Record<NodeId, Stage>,
  nodes: Record<NodeId, Node>
): Promise<Stuff> {
  let nextTensorId = count();
  const visited: NodeId[] = [];
  const dependencies: Record<NodeId, NodeDependencies> = {};

  const tensorConstraints = await allGraphs(pipeline, nodes);

  const inputs = Object.entries(pipeline)
    .filter(([_, stage]) => isCapabilityStage(stage))
    .map(([name, _]) => name);
  const toVisit = Object.entries(pipeline)
    .filter(([_, stage]) => isOutStage(stage))
    .map(([name, _]) => name);

  // assume each capability node has 1 input
  for (const name of inputs) {
    const id = nextTensorId();
    dependencies[name] = {
      inputs: { [`${name}.0`]: id },
      outputs: { [`${name}.0`]: id },
    };
  }

  let node;

  while ((node = toVisit.pop())) {
    visited.push(node);
    const stage = pipeline[node];

    if (isCapabilityStage(stage) || isOutStage(stage)) {
      continue;
    }

    const outputs = tensorConstraints[node].outputs.map(
      (desc) => [desc.name, nextTensorId()] as const
    );
    dependencies[node] = {
      inputs: {},
      outputs: Object.fromEntries(outputs),
    };

    stageInputs(stage)
      .filter(({ node }) => !visited.includes(node))
      .forEach(({ node }) => toVisit.push(node));
  }

  for (const [stageName, stage] of Object.entries(pipeline)) {
    const inputs = stageInputs(stage);

    inputs.forEach(({ node, index }, i) => {
      const { name: previousTensorName } =
        tensorConstraints[node].outputs[index];
      const { name: currentTensorName } =
        tensorConstraints[stageName].inputs[i];
      dependencies[stageName].inputs[currentTensorName] =
        dependencies[node].outputs[previousTensorName];
    });
  }

  visited.reverse();

  return {
    dependencies,
    evaluationOrder: visited,
  };
}

export class Runtime {
  /**
   * The tensors associated with each node.
   */
  private tensors: Record<TensorId, Tensor> = {};
  private logger: Logger;

  constructor(
    private doc: DocumentV1,
    private nodes: Record<NodeId, Node>,
    private dependencies: Record<NodeId, NodeDependencies>,
    private evaluationOrder: NodeId[],
    logger: Logger
  ) {
    this.logger = logger.child({ name: "Runtime" });
  }

  public async infer(): Promise<void> {
    this.logger.debug("Starting inference");
    const start = Date.now();

    for (const name of this.evaluationOrder) {
      this.evaluateNode(name);
    }

    const durationMs = Date.now() - start;
    this.logger.debug({ durationMs }, "Inference completed successfully");
  }

  public get inputs(): string[] {
    const inputs: string[] = [];

    for (const [name, stage] of Object.entries(this.doc.pipeline)) {
      // TODO: check for proc-blocks with no input tensors in the Runefile
      if (isCapabilityStage(stage)) {
        inputs.push(name);
      }
    }

    return inputs;
  }

  public setNodeInput(node: string, name: string, tensor: Tensor) {
    if (!(node in this.dependencies)) {
      throw new Error();
    }

    const { inputs } = this.dependencies[node];

    if (!(name in inputs)) {
      throw new Error();
    }

    const id = inputs[name];
    this.tensors[id] = tensor;
  }

  private async evaluateNode(name: string) {
    if (name in this.tensors) {
      // already been evaluated
      return;
    }

    if (!(name in this.nodes)) {
      throw new Error(`No "${name}" node registered`);
    }
    if (!(name in this.doc.pipeline)) {
      throw new Error(`The Runefile doesn't contain a "${name}" node`);
    }

    const stage = this.doc.pipeline[name];

    if (isOutStage(stage)) {
      // output stages don't do anything. Note: we will be deleting output
      // stages altogether.
      return;
    }

    this.logger.debug({ node: name }, "Evaluating a node");
    const start = Date.now();

    const node = this.nodes[name];
    const args = stageArguments(this.doc.pipeline[name]);

    const inputs = this.nodeInputs(name);

    const outputs = await node.infer(inputs, args);

    this.setNodeOutputs(name, outputs);

    const durationMs = Date.now() - start;
    this.logger.debug({ durationMs, node: name }, "Node evaluated");
    this.logger.trace({ inputs, outputs, args, node: name });
  }

  private setNodeOutputs(
    name: string,
    outputs: Record<string, runtime_v1.Tensor>
  ) {
    for (const [tensorName, id] of Object.entries(
      this.dependencies[name].outputs
    )) {
      this.tensors[id] = outputs[tensorName];
    }
  }

  private nodeInputs(node: string): Record<string, Tensor> {
    const tensors: Record<string, Tensor> = {};

    for (const [name, id] of Object.entries(this.dependencies[node].inputs)) {
      if (!(id in this.tensors)) {
        throw new Error(
          `The "${node}" node requires tensor ${id}, but it hasn't been set`
        );
      }

      tensors[name] = this.tensors[id];
    }

    return tensors;
  }
}

function procBlockNodes(
  procBlocks: Record<string, ProcBlockLike>
): Record<string, ProcBlockNode> {
  const nodes: Record<string, ProcBlockNode> = {};

  for (const [name, procBlock] of Object.entries(procBlocks)) {
    nodes[name] = new ProcBlockNode(procBlock);
  }

  return nodes;
}

/**
 * An adapter class that makes each ProcBlock method asynchronous.
 */
class ProcBlockNode implements Node {
  constructor(private procBlock: ProcBlockLike) {}

  graph(args: Record<string, string>): Promise<Tensors> {
    const tensors = this.procBlock.graph(args);
    return Promise.resolve(tensors);
  }

  infer(
    inputs: Record<string, runtime_v1.Tensor>,
    args: Record<string, string>
  ): Promise<Record<string, runtime_v1.Tensor>> {
    const outputs = this.procBlock.evaluate(inputs, args);
    return Promise.resolve(outputs);
  }
}

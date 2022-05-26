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

type NodeId = string;

interface ProcBlockLike {
  graph(args: Record<string, string>): Tensors;
  evaluate(
    inputs: Record<string, runtime_v1.Tensor>,
    args: Record<string, string>
  ): Record<string, runtime_v1.Tensor>;
}

export function create(
  doc: DocumentV1,
  procBlocks: Record<string, ProcBlockLike>,
  models: Record<string, Node>
): Runtime {
  const pb = procBlockNodes(procBlocks);
  const nodes: Record<string, Node> = { ...models, ...pb };

  return new Runtime(doc, nodes);
}

type NamedTensor = {
  parentNode: NodeId;
  outputIndex: number;
} & Tensor;

export class Runtime {
  outputTensors: Record<string, Tensor[]> = {};
  private inputTensors: Record<string, Tensor> = {};
  private tensors: NamedTensor[] = [];

  constructor(private doc: DocumentV1, private nodes: Record<string, Node>) {}

  public async infer(): Promise<void> {
    // drop any existing tensors
    this.tensors = [];
    this.outputTensors = {};

    for (const [name, stage] of Object.entries(this.doc.pipeline)) {
      if (isOutStage(stage)) {
        // Output stages are the terminal nodes in our DAG. They aren't
        // actually backed by anything, so we just need to (recursively)
        // evaluate the node's inputs.
        await this.evaluatePrerequisites(stage);

        const inputs = stageInputs(stage).map(({ node, index }) => {
          const tensor = this.findTensor(node, index);
          if (!tensor) {
            throw new Error(
              `The "${node}.${index}" tensor wasn't found (needed by "${name}")`
            );
          }
          return tensor;
        });

        const results = await Promise.all(inputs);
        this.outputTensors[name] = results.map(
          ({ buffer, dimensions, elementType }) => ({
            buffer,
            dimensions,
            elementType,
          })
        );
      }
    }
  }

  public get inputs(): string[] {
    const inputs: string[] = [];

    for (const [name, stage] of Object.entries(this.doc.pipeline)) {
      if (isCapabilityStage(stage)) {
        inputs.push(name);
      }
    }

    return inputs;
  }

  public setInput(node: string, tensor: Tensor) {
    this.inputTensors[node] = tensor;
  }

  private async evaluatePrerequisites(stage: Stage) {
    const inputNames = stageInputs(stage).map((input) => input.node);

    const deduplicatedNames = new Set(inputNames);

    const promises: Array<Promise<void>> = [];
    deduplicatedNames.forEach((name) => {
      const alreadyEvaluated = this.tensors.some((t) => t.parentNode == name);

      if (!alreadyEvaluated) {
        promises.push(this.evaluateNode(name));
      }
    });

    await Promise.all(promises);
  }

  private findTensor(
    parentNode: string,
    outputIndex: number
  ): NamedTensor | undefined {
    return this.tensors.find(
      (t) => t.parentNode == parentNode && t.outputIndex == outputIndex
    );
  }

  private async evaluateNode(name: string) {
    if (!(name in this.nodes)) {
      throw new Error(`No "${name}" node registered`);
    }
    if (!(name in this.doc.pipeline)) {
      throw new Error(`The Runefile doesn't contain a "${name}" node`);
    }

    const stage = this.doc.pipeline[name];
    await this.evaluatePrerequisites(stage);

    const node = this.nodes[name];
    const args = stageArguments(this.doc.pipeline[name]);

    // Fixme: this could be computed once and cached.
    const { inputs: inputDescriptors, outputs: outputDescriptors } =
      await node.graph(args);

    const inputs = isCapabilityStage(stage)
      ? this.getInputTensorsForInputNode(name, inputDescriptors)
      : this.getNonInputNodeInputTensors(stage, inputDescriptors);

    const outputs = await node.infer(inputs, args);

    const outputTensors = outputDescriptors.map((descriptor, i) => ({
      ...outputs[descriptor.name],
      parentNode: name,
      outputIndex: i,
    }));
    this.tensors.push(...outputTensors);
  }

  private getInputTensorsForInputNode(
    name: string,
    inputs: TensorDescriptor[]
  ): Record<string, Tensor> {
    if (!(name in this.inputTensors)) {
      throw new Error(`The "${name}" input tensor wasn't set`);
    }

    let tensorName;

    switch (inputs.length) {
      case 0:
        tensorName = name;
        break;
      case 1:
        tensorName = inputs[0].name;
        break;
      default:
        throw new Error(
          `The "${name}" node is an input and should only accept 1 tensor, found ${inputs.length}`
        );
    }

    return { [tensorName]: this.inputTensors[name] };
  }

  private getNonInputNodeInputTensors(
    stage: Stage,
    inputDescriptors: TensorDescriptor[]
  ): Record<string, Tensor> {
    const inputs = stageInputs(stage).map(({ node, index }, inputIndex) => {
      const tensor = this.findTensor(node, index);
      if (!tensor) {
        throw new Error(`The "${node}.${index}" tensor hasn't been evaluated`);
      }

      const inputName = inputDescriptors[inputIndex].name;
      return [inputName, tensor] as const;
    });

    return Object.fromEntries(inputs);
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

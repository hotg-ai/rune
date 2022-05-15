import type { Tensor } from ".";
import { Runtime as RuntimeInterface } from "./loader";
import { Pipeline } from "./loader/pipeline";
import { StructuredLogger, Logger } from "./logging";
import { TensorDescriptor } from "./proc_blocks";

type TensorId = number;
type NodeId = number;

class Runtime implements RuntimeInterface {
  private tensors: Record<TensorId, Tensor> = {};

  constructor(private pipeline: Pipeline, private logger: StructuredLogger) {}

  public async infer(): Promise<void> {
    const span = this.logger.span("infer");
    const start = Date.now();
    span.info("Started evaluating the Rune");

    for (const id of this.pipeline.evaluationOrder) {
      const start = Date.now();
      await this.evaluate(id);

      span.debug("Evaluated node", {
        durationMs: Date.now() - start,
        name: this.pipeline.nodeInfo[id].name,
      });
    }

    span.debug("Evaluation complete", { durationMs: Date.now() - start });
  }

  public inputs(): Record<string, TensorDescriptor> {
    const entries = this.pipeline.inputs
      .map((id) => this.pipeline.nodeInfo[id])
      .map((info) => {
        const [id] = Object.values(info.inputs);
        return [info.name, this.pipeline.tensors[id]];
      });

    return Object.fromEntries(entries);
  }

  public setInput(name: string, tensor: Tensor) {
    const node = Object.values(this.pipeline.nodeInfo).find(
      (n) => n.name == name
    );

    if (!node) {
      this.logger.error("Trying to set an input on an unknown node", { name });
      return;
    }

    const entries = Object.entries(node.inputs);

    if (entries.length != 1) {
      this.logger.error(
        "Unable to set the input for a node with multiple input tensors",
        {
          name,
          inputTensors: entries.length,
        }
      );
      return;
    }

    const [_, id] = entries[0];
    this.tensors[id] = tensor;
  }

  private async evaluate(id: NodeId) {
    const node = this.pipeline.nodes[id];
    const info = this.pipeline.nodeInfo[id];

    const inputs = this.getTensorsById(info.inputs);
    const outputs = await node.infer(inputs, info.args);
    this.tensors = { ...this.tensors, ...outputs };
  }

  private getTensorsById(
    ids: Record<string, TensorId>
  ): Record<string, Tensor> {
    const tensors: Record<TensorId, Tensor> = {};

    for (const [name, id] of Object.entries(ids)) {
      if (id in this.tensors) {
        tensors[id] = this.tensors[id];
      } else {
        throw new Error(
          `Tried to look up a non-existent tensor with ID ${id} ("${name}")`
        );
      }
    }

    return tensors;
  }
}

export function createRuntime(
  pipeline: Pipeline,
  logger: Logger
): RuntimeInterface {
  return new Runtime(pipeline, new StructuredLogger(logger, "Runtime"));
}

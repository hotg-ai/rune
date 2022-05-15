import { proc_block_v1, runtime_v1 } from "@hotg-ai/rune-wit-files";
import type { Metadata, TensorDescriptor } from ".";
import { Logger, StructuredLogger } from "../logging";
import { GraphContext, HostFunctions, KernelContext } from "./HostFunctions";

type ProcBlockBuffer = Parameters<proc_block_v1.ProcBlockV1["instantiate"]>[0];

type Tensors = {
  inputs: TensorDescriptor[];
  outputs: TensorDescriptor[];
};

/**
 * An executable proc-block.
 */
export class ProcBlock {
  private constructor(
    private hostFunctions: HostFunctions,
    private instance: proc_block_v1.ProcBlockV1,
  ) {}

  static async load(
    procBlock: ProcBlockBuffer,
    logger: Logger
  ): Promise<ProcBlock> {
    const log = new StructuredLogger(logger, "ProcBlock");

    const span = log.span("load");
    span.info("Loading the proc-block");
    const start = Date.now();

    const wrapper = new proc_block_v1.ProcBlockV1();
    const imports: any = {};

    const hostFunctions = new HostFunctions(logger);
    runtime_v1.addRuntimeV1ToImports(
      imports,
      hostFunctions,
      (name) => wrapper.instance.exports[name]
    );

    await wrapper.instantiate(procBlock, imports);

    span.debug("Finished loading the proc-block", {
      durationMs: Date.now() - start,
    });

    return new ProcBlock(hostFunctions, wrapper);
  }

  /**
   * Extract metadata from the proc-block.
   */
  metadata(): Metadata | undefined {
    this.hostFunctions.metadata = undefined;
    this.instance.registerMetadata();
    return this.hostFunctions.metadata;
  }

  /**
   * Given the provided set of arguments, what would this proc-block's input
   * and output tensors be?
   */
  graph(args: Record<string, string>): Tensors {
    const ctx = new GraphContext(args);
    this.hostFunctions.graph = ctx;
    this.instance.graph("");
    const { inputs, outputs } = ctx;
    return { inputs, outputs };
  }

  /**
   * Evaluate this proc-block.
   *
   * @param args Key-value arguments that control the proc-block's behaviour.
   * @param inputs Input tensors.
   * @returns
   */
  evaluate(
    args: Record<string, string>,
    inputs: Record<string, runtime_v1.Tensor>
  ): Record<string, runtime_v1.Tensor> {
    const ctx = new KernelContext(args, inputs);
    this.hostFunctions.kernel = ctx;
    this.instance.kernel("");
    return ctx.outputs;
  }
}

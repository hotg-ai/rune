import { proc_block_v1, runtime_v1 } from "@hotg-ai/rune-wit-files";
import type { Metadata, Tensors } from ".";
import { Logger, StructuredLogger } from "../logging";
import { GraphContext, HostFunctions, KernelContext } from "./HostFunctions";

type ProcBlockBinary = Parameters<proc_block_v1.ProcBlockV1["instantiate"]>[0];

/**
 * An executable proc-block.
 */
export class ProcBlock {
  private constructor(
    private hostFunctions: HostFunctions,
    private instance: proc_block_v1.ProcBlockV1
  ) {}

  static async load(wasm: ProcBlockBinary, logger: Logger): Promise<ProcBlock> {
    const log = new StructuredLogger(logger, "ProcBlock");

    const span = log.span("load");
    span.info("Loading the proc-block");
    const start = Date.now();

    const procBlock = new proc_block_v1.ProcBlockV1();
    const imports: any = {};

    const hostFunctions = new HostFunctions(logger);
    runtime_v1.addRuntimeV1ToImports(
      imports,
      hostFunctions,
      (name) => procBlock.instance.exports[name]
    );

    await procBlock.instantiate(wasm, imports);

    span.debug("Finished loading the proc-block", {
      durationMs: Date.now() - start,
    });

    return new ProcBlock(hostFunctions, procBlock);
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
    const result = this.instance.graph("");

    if (result.tag == "err") {
      handleGraphError(result.val);
    }

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
    inputs: Record<string, runtime_v1.Tensor>,
    args: Record<string, string>
  ): Record<string, runtime_v1.Tensor> {
    const ctx = new KernelContext(args, inputs);
    this.hostFunctions.kernel = ctx;

    const result = this.instance.kernel("");

    if (result.tag == "err") {
      handleKernelError(result.val);
    }

    return ctx.outputs;
  }
}

function handleGraphError(err: proc_block_v1.GraphError): never {
  switch (err.tag) {
    case "invalid-argument":
      const { name, reason } = err.val;
      handleInvalidArgument(name, reason);

    case "missing-context":
      throw new Error("The proc-block couldn't access the context object");

    case "other":
      throw new Error(err.val);
  }
}

function handleKernelError(err: proc_block_v1.KernelError): never {
  switch (err.tag) {
    case "invalid-input":
      const { name, reason } = err.val;
      handleInvalidInput(name, reason);

    default:
      handleGraphError(err);
  }
}

function handleInvalidInput(
  name: string,
  reason: proc_block_v1.BadInputReason
): never {
  switch (reason.tag) {
    case "invalid-value":
      throw new Error(
        `The "${name}" input had an invalid value: ${reason.val}`
      );

    case "unsupported-shape":
      throw new Error(`The "${name}" input had a the wrong shape`);

    case "not-found":
      throw new Error(`The "${name}" input wasn't set`);

    case "other":
      throw new Error(`The "${name}" input was invalid: ${reason.val}`);
  }
}

function handleInvalidArgument(
  name: string,
  reason: proc_block_v1.BadArgumentReason
): never {
  switch (reason.tag) {
    case "invalid-value":
      throw new Error(
        `The "${name}" argument had an invalid value: ${reason.val}`
      );

    case "not-found":
      throw new Error(`The "${name}" argument wasn't set`);

    case "other":
      throw new Error(`The "${name}" argument was invalid: ${reason.val}`);
  }
}

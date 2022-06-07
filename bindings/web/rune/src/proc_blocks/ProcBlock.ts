import { proc_block_v1, runtime_v1 } from "@hotg-ai/rune-wit-files";
import { Logger } from "pino";
import type { Metadata, Tensors } from ".";
import { GraphContext, HostFunctions, KernelContext } from "./HostFunctions";

type ProcBlockBinary = Parameters<proc_block_v1.ProcBlockV1["instantiate"]>[0];

/**
 * An executable proc-block.
 */
export class ProcBlock {
  private constructor(
    private hostFunctions: HostFunctions,
    private instance: proc_block_v1.ProcBlockV1,
    private logger: Logger
  ) {}

  /**
   * Load a ProcBlock from a WebAssembly module.
   *
   * @param wasm Something that can be used to instantiate a WebAssembly module.
   * @param rootLogger A logger that this ProcBlock can use.
   * @returns
   */
  static async load(
    wasm: ProcBlockBinary,
    rootLogger: Logger
  ): Promise<ProcBlock> {
    // Note: We want the host functions logger to have a different "name" field
    // to the ProcBlock object.
    const hostFunctionsLogger = rootLogger.child({ name: "HostFunctions" });
    const logger = rootLogger.child({ name: "ProcBlock" });

    logger.info("Loading the proc-block");
    const start = Date.now();

    const procBlock = new proc_block_v1.ProcBlockV1();
    const imports: any = {};

    const hostFunctions = new HostFunctions(hostFunctionsLogger);
    runtime_v1.addRuntimeV1ToImports(
      imports,
      hostFunctions,
      (name) => procBlock.instance.exports[name]
    );

    await procBlock.instantiate(wasm, imports);

    const durationMs = Date.now() - start;
    rootLogger.debug({ durationMs }, "Finished loading the proc-block");

    return new ProcBlock(hostFunctions, procBlock, logger);
  }

  /**
   * Extract metadata from the proc-block.
   */
  metadata(): Metadata {
    this.hostFunctions.metadata = undefined;
    this.instance.registerMetadata();

    if (!this.hostFunctions.metadata) {
      throw new Error("The proc-block didn't register any metadata");
    }
    return this.hostFunctions.metadata;
  }

  /**
   * Given the provided set of arguments, what would this proc-block's input
   * and output tensors be?
   */
  graph(args: Record<string, string>): Tensors {
    this.logger.debug({ args }, "Calling the graph function");

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
    this.logger.debug(
      { args, inputs: Object.keys(inputs) },
      "Evaluating a proc-block"
    );

    const ctx = new KernelContext(args, inputs);
    this.hostFunctions.kernel = ctx;

    const result = this.instance.kernel("");

    if (result.tag == "err") {
      this.logger.error(
        { error: result.val },
        "Evaluating the proc-block failed"
      );
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

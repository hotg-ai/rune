import type { ModelHandler, Runtime } from ".";
import { consoleLogger, Logger, StructuredLogger } from "../logging";

export class RuneLoader {
  public static default: RuneLoader = new RuneLoader()
    .withLogger(consoleLogger);

  private modelHandlers: Record<string, ModelHandler> = {};
  private logger: Logger = { log: () => {}, isEnabled: () => false };

  /**
   * Set the logger that will be used whenever the Rune emits a message.
   */
  public withLogger(logger: Logger | Logger["log"]): this {
    if (typeof logger == "function") {
      // As a convenience, we let people pass in a logging function if
      // they don't care about isEnabled().
      this.logger = { log: logger, isEnabled: m => m.level != "trace" };
    } else {
      this.logger = logger;
    }

    return this;
  }

  public withModelHandler(modelType: string, handler: ModelHandler): this {
    this.modelHandlers[modelType] = handler;
    return this;
  }

  /**
   * Load the Rune, instantiating a Runtime that can be used to interact with
   * it.
   *
   * @param rune
   */
  public async load(rune: ArrayBuffer): Promise<Runtime> {
    const log = new StructuredLogger(this.logger, RuneLoader.name);

    log.info("Loading the Rune", { bytes: rune.byteLength });

    throw new Error();
  }
}

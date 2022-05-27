import { Logger, pino } from "pino";
import type { ModelHandler, Runtime } from ".";
import { RuneLoader } from "./RuneLoader";

/**
 * A builder object that lets you configure how a Rune is loaded.
 */
export class Rune {
  private modelHandlers: Record<string, ModelHandler> = {};
  private logger: Logger = pino({ level: "silent", enabled: false });

  /**
   * Set the logger that will be used during the loading process and by the
   * Rune runtime.
   */
  public withLogger(logger: Logger): this {
    this.logger = logger;
    return this;
  }

  /**
   * Register a model handler based on the "model-format" argument attached to
   * a model node.
   */
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
  public async load(rune: Uint8Array): Promise<Runtime> {
    const loader = new RuneLoader(this.modelHandlers, this.logger);
    return await loader.load(rune);
  }
}

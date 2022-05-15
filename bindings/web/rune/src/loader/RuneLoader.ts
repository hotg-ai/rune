import JSZip from "jszip";
import yaml from "js-yaml";
import type { ModelHandler, Runtime } from ".";
import { consoleLogger, Logger, StructuredLogger } from "../logging";
import {
  CapabilityStage,
  DocumentV1,
  ModelStage,
  OutStage,
  ProcBlockStage,
  Stage,
} from "../Runefile";
import { createRuntime } from "../Runtime";
import { ProcBlock } from "../proc_blocks";
import { determinePipeline } from "./Pipeline";

export class RuneLoader {
  public static default: RuneLoader = new RuneLoader().withLogger(
    consoleLogger
  );

  private modelHandlers: Record<string, ModelHandler> = {};
  private logger: Logger = { log: () => {}, isEnabled: () => false };

  /**
   * Set the logger that will be used whenever the Rune emits a message.
   */
  public withLogger(logger: Logger | Logger["log"]): this {
    if (typeof logger == "function") {
      // As a convenience, we let people pass in a logging function if
      // they don't care about isEnabled().
      this.logger = { log: logger, isEnabled: (m) => m.level != "trace" };
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
  public async load(rune: Uint8Array): Promise<Runtime> {
    const log = new StructuredLogger(this.logger, RuneLoader.name);

    log.info("Loading the Rune", { bytes: rune.byteLength });

    const zip = new JSZip();
    await zip.loadAsync(rune);

    const f = zip.file("Runefile.yml");
    if (!f) {
      throw new Error("No Runefile.yml found");
    }
    const src = await f.async("string");
    const runefile = yaml.load(src);

    if (!isRunefile(runefile)) {
      throw new Error("Invalid Runefile");
    }

    log.debug("Parsed the Runefile", { length: src.length });

    const nodes = splitByStageType(runefile);
    const procBlocks = await instantiateProcBlocks(
      nodes.procBlock,
      zip,
      log.span("instantiate-proc-blocks")
    );
    const models = await loadModels(
      nodes.model,
      zip,
      log.span("instantiate-models"),
      this.modelHandlers
    );

    const pipeline = determinePipeline(runefile);

    return createRuntime(pipeline, this.logger);
  }
}

function isRunefile(value?: any): value is DocumentV1 {
  return value && value.version == "1" && value.pipeline && value.image;
}

type Stages = {
  capability: Record<string, CapabilityStage>;
  procBlock: Record<string, ProcBlockStage>;
  model: Record<string, ModelStage>;
  out: Record<string, OutStage>;
};

function splitByStageType(runefile: DocumentV1): Stages {
  const nodes: Stages = { capability: {}, procBlock: {}, model: {}, out: {} };

  for (const [name, stage] of Object.entries(runefile.pipeline)) {
    if (isProcBlockStage(stage)) {
      nodes.procBlock[name] = stage;
    } else if (isModelStage(stage)) {
      nodes.model[name] = stage;
    } else if (isCapabilityStage(stage)) {
      nodes.capability[name] = stage;
    } else if (isOutStage(stage)) {
      nodes.out[name] = stage;
    }
  }

  return nodes;
}

async function instantiateProcBlocks(
  stages: Record<string, ProcBlockStage>,
  zip: JSZip,
  log: StructuredLogger
): Promise<Record<string, ProcBlock>> {
  const start = Date.now();

  const promises = Object.entries(stages).map(async ([name, stage]) => {
    const filename = stage["proc-block"];
    log.debug("Reading proc-block", { name, filename });

    const file = zip.file(filename);

    if (!file) {
      throw new Error(`The Rune doesn't contain "${filename}"`);
    }

    const data = await file.async("arraybuffer");
    return [name, await ProcBlock.load(data, log.backend)];
  });

  const procBlocks = Object.fromEntries(await Promise.all(promises));

  log.debug("Finished instantiating all proc-blocks", {
    count: Object.keys(procBlocks).length,
    durationMs: Date.now() - start,
  });

  return procBlocks;
}

async function loadModels(
  stages: Record<string, ModelStage>,
  zip: JSZip,
  log: StructuredLogger,
  modelHandlers: Record<string, ModelHandler>
): Promise<Record<string, Node>> {
  const start = Date.now();

  const promises = Object.entries(stages).map(async ([name, stage]) => {
    const format = stage.args?.["model-format"] || "tensorflow-lite";
    const filename = stage.model;
    log.debug("Loading model", { name, format, filename });

    const file = zip.file(filename);

    if (!file) {
      throw new Error(`The Rune doesn't contain "${filename}"`);
    }

    if (!(format in modelHandlers)) {
      throw new Error(
        `No handler was registered for the "${format}" model on the "${name}" node`
      );
    }

    const handler = modelHandlers[format];

    const data = await file.async("arraybuffer");
    const model = await handler(data, translateArgs(stage.args));

    log.debug("Loaded model", { name, length: data.byteLength });

    return [name, model];
  });

  const models = Object.fromEntries(await Promise.all(promises));

  log.debug("Finished instantiating all models", {
    count: Object.keys(models).length,
    durationMs: Date.now() - start,
  });

  return models;
}

function isModelStage(stage: Stage): stage is ModelStage {
  return "model" in stage;
}

function isCapabilityStage(stage: Stage): stage is CapabilityStage {
  return "capability" in stage;
}

function isProcBlockStage(stage: Stage): stage is ProcBlockStage {
  return "proc-block" in stage;
}

function isOutStage(stage: Stage): stage is OutStage {
  return "out" in stage;
}

function translateArgs(
  args?: Record<string, string | number>
): Record<string, string> {
  if (!args) {
    return {};
  }

  const entries = Object.entries(args).map(([key, value]) => [
    key,
    value.toString(),
  ]);

  return Object.fromEntries(entries);
}

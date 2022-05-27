import JSZip from "jszip";
import yaml from "js-yaml";
import { Logger, pino } from "pino";
import type { ModelHandler, Node, Runtime, Tensor } from ".";
import {
  CapabilityStage,
  DocumentV1,
  ModelStage,
  OutStage,
  ProcBlockStage,
} from "./Runefile";
import { ProcBlock } from "./proc_blocks";
import { create } from "./Runtime";
import {
  isCapabilityStage,
  isModelStage,
  isOutStage,
  isProcBlockStage,
  isRunefile,
  stageArguments,
} from "./utils";


export class RuneLoader {
  logger: Logger;

  constructor(
    private modelHandlers: Record<string, ModelHandler>,
    private rootLogger: Logger
  ) {
    this.logger = rootLogger.child({ name: "RuneLoader" });
  }

  async load(rune: Uint8Array): Promise<Runtime> {
    this.logger.info({ bytes: rune.byteLength }, "Loading the Rune");

    const zip = new JSZip();
    await zip.loadAsync(rune);
    const runefile = await this.parseRunefile(zip);

    const nodes = splitByStageType(runefile);
    const procBlocks = await this.instantiateProcBlocks(nodes, zip);
    const models = await this.loadModels(nodes.model, zip, this.modelHandlers);

    return create(runefile, procBlocks, models, this.rootLogger);
  }

  async parseRunefile(zip: JSZip): Promise<DocumentV1> {
    const f = zip.file("Runefile.yml");
    if (!f) {
      throw new Error("No Runefile.yml found");
    }
    const src = await f.async("string");
    const runefile = yaml.load(src);

    if (!isRunefile(runefile)) {
      throw new Error("Invalid Runefile");
    }

    this.logger.debug({ length: src.length }, "Parsed the Runefile");

    return runefile;
  }

  async instantiateProcBlocks(
    stages: Stages,
    zip: JSZip
  ): Promise<Record<string, ProcBlock>> {
    const start = Date.now();

    const entries = stagesBackedByProcBlocks(stages).map(
      async ({ name, path }) => {
        this.logger.debug({ procBlock: name, path }, "Reading proc-block");

        const file = zip.file(path);

        if (!file) {
          throw new Error(`The Rune doesn't contain "${path}"`);
        }

        const data = await file.async("arraybuffer");
        const procBlock = await ProcBlock.load(
            data,
            this.rootLogger.child({ procBlock: name })
          );
        return [ name, procBlock ] as const;
      }
    );

    const procBlocks = Object.fromEntries(await Promise.all(entries));

    this.logger.debug({
      count: Object.keys(procBlocks).length,
      durationMs: Date.now() - start,
    }, "Finished instantiating all proc-blocks");

    return procBlocks;
  }

  async loadModels(
    stages: Record<string, ModelStage>,
    zip: JSZip,
    modelHandlers: Record<string, ModelHandler>
  ): Promise<Record<string, Node>> {
    const start = Date.now();

    const promises = Object.entries(stages).map(async ([name, stage]) => {
      const format = stage.args?.["model-format"] || "tensorflow-lite";
      const filename = stage.model;
      this.logger.debug({ model: name, format, filename }, "Loading model");

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
      const model = await handler(data, stageArguments(stage), this.rootLogger);

      this.logger.debug(
        { model: name, length: data.byteLength },
        "Loaded model"
      );

      return [name, model];
    });

    const models = Object.fromEntries(await Promise.all(promises));

    this.logger.debug(
      {
        count: Object.keys(models).length,
        durationMs: Date.now() - start,
      },
      "Finished instantiating all models"
    );

    return models;
  }
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

function stagesBackedByProcBlocks(stages: Stages) {
  const procBlocks: Array<{ name: string; path: string }> = [];

  for (const [name, stage] of Object.entries(stages.procBlock)) {
    const path = stage["proc-block"];
    procBlocks.push({ name, path });
  }

  for (const [name, stage] of Object.entries(stages.capability)) {
    const path = stage.capability;
    procBlocks.push({ name, path });
  }

  return procBlocks;
}

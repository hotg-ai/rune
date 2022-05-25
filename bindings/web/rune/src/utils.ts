import { ElementType, Tensor } from ".";
import {
  CapabilityStage,
  DocumentV1,
  ModelStage,
  OutStage,
  ProcBlockStage,
  Stage,
} from "./Runefile";

export function isModelStage(stage: Stage): stage is ModelStage {
  return "model" in stage;
}

export function isCapabilityStage(stage: Stage): stage is CapabilityStage {
  return "capability" in stage;
}

export function isProcBlockStage(stage: Stage): stage is ProcBlockStage {
  return "proc-block" in stage;
}

export function isOutStage(stage: Stage): stage is OutStage {
  return "out" in stage;
}

export function isRunefile(value?: any): value is DocumentV1 {
  return value && value.version == "1" && value.pipeline && value.image;
}

export function stageArguments({ args }: Stage): Record<string, string> {
  if (!args) {
    return {};
  }

  const entries = Object.entries(args).map(([key, value]) => [
    key,
    value.toString(),
  ]);

  return Object.fromEntries(entries);
}

export type InputName = {
  node: string;
  index: number;
};

export function stageInputs(stage: Stage): InputName[] {
  if (isCapabilityStage(stage) || !stage.inputs) {
    return [];
  }

  return stage.inputs.map(parsePortId);
}

function parsePortId(value: string): InputName {
  const match = value.match(/^[\w\d_-]+(?:\.(\d+))?$/);

  if (!match) {
    throw new Error(`Unable to parse the input, "${value}"`);
  }

  const node = match[0];
  const index = match[1] ? parseInt(match[1]) : 0;

  return { node, index };
}

export function floatTensor(values: number[]): Tensor {
  const floats = Float32Array.from(values);
  return {
    elementType: ElementType.F32,
    dimensions: Uint32Array.from([1, 1]),
    buffer: new Uint8Array(floats.buffer),
  };
}

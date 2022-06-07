import { runtime_v1 } from "@hotg-ai/rune-wit-files";
import { Logger, Level, levels } from "pino";
import type {
  ArgumentHint,
  ArgumentMetadata,
  Metadata,
  SupportedShapes,
  TensorHint,
  TensorMetadata,
  TensorDescriptor,
} from ".";

const logLevels: Record<runtime_v1.LogLevel, Level> = {
  [runtime_v1.LogLevel.Trace]: "trace",
  [runtime_v1.LogLevel.Debug]: "debug",
  [runtime_v1.LogLevel.Info]: "info",
  [runtime_v1.LogLevel.Warn]: "warn",
  [runtime_v1.LogLevel.Error]: "error",
  [runtime_v1.LogLevel.Fatal]: "fatal",
};

export class HostFunctions implements runtime_v1.RuntimeV1 {
  metadata?: Metadata;
  graph?: GraphContext;
  kernel?: KernelContext;

  constructor(private logger: Logger) { }

  metadataNew(name: string, version: string): runtime_v1.Metadata {
    return new MetadataBuilder({
      name,
      version,
      arguments: [],
      inputs: [],
      outputs: [],
      tags: [],
    });
  }

  argumentMetadataNew(name: string): runtime_v1.ArgumentMetadata {
    return new ArgumentMetadataBuilder({ name, hints: [] });
  }

  tensorMetadataNew(name: string): runtime_v1.TensorMetadata {
    return new TensorMetadataBuilder({ name, hints: [] });
  }

  interpretAsImage(): TensorHint {
    return { type: "media-hint", media: "image" };
  }

  interpretAsAudio(): TensorHint {
    return { type: "media-hint", media: "audio" };
  }

  supportedShapes(
    supportedElementTypes: runtime_v1.ElementType[],
    dimensions: runtime_v1.Dimensions
  ): SupportedShapes {
    return {
      type: "supported-shapes",
      supportedElementTypes,
      dimensions,
    };
  }

  interpretAsNumberInRange(min: string, max: string): ArgumentHint {
    return { type: "number-in-range", min, max };
  }

  interpretAsStringInEnum(stringEnum: string[]): ArgumentHint {
    return { type: "string-enum", possibleValues: stringEnum };
  }

  nonNegativeNumber(): ArgumentHint {
    return { type: "non-negative-number" };
  }

  supportedArgumentType(hint: runtime_v1.ArgumentType): ArgumentHint {
    return { type: "supported-argument-type", argumentType: hint };
  }

  registerNode(metadata: runtime_v1.Metadata): void {
    if (metadata instanceof MetadataBuilder) {
      this.metadata = metadata.meta;
    }
  }

  graphContextForNode(nodeId: string): runtime_v1.GraphContext | null {
    return this.graph || null;
  }

  kernelContextForNode(nodeId: string): runtime_v1.KernelContext | null {
    return this.kernel || null;
  }

  isEnabled(meta: runtime_v1.LogMetadata): boolean {
    const requestedLevel = logLevels[meta.level];
    const threshold = this.logger.levelVal;
    return levels.values[requestedLevel] > threshold;
  }

  log(
    metadata: runtime_v1.LogMetadata,
    message: string,
    data: runtime_v1.LogValueMap
  ): void {
    const payload = data.map(([key, value]) => {
      return value.tag == "null" ? [key, null] : [key, value.val];
    });

    const level = logLevels[metadata.level];
    const log = this.logger[level];

    log({ metadata, payload: Object.fromEntries(payload) }, message);
  }

  modelLoad(
    modelFormat: string,
    model: Uint8Array,
    args: [string, string][]
  ): runtime_v1.Result<runtime_v1.Model, runtime_v1.ModelLoadError> {
    throw new Error("Method not implemented.");
  }
}

class MetadataBuilder implements runtime_v1.Metadata {
  constructor(public meta: Metadata) {}

  setDescription(description: string): void {
    this.meta.description = description;
  }

  setRepository(url: string): void {
    if (url) {
      this.meta.repository = url;
    }
  }

  setHomepage(url: string): void {
    if (url) {
      this.meta.homepage = url;
    }
  }

  addTag(tag: string): void {
    this.meta.tags.push(tag);
  }

  addArgument(arg: runtime_v1.ArgumentMetadata): void {
    if (arg instanceof ArgumentMetadataBuilder) {
      this.meta.arguments.push(arg.meta);
    }
  }

  addInput(metadata: runtime_v1.TensorMetadata): void {
    if (metadata instanceof TensorMetadataBuilder) {
      this.meta.inputs.push(metadata.meta);
    }
  }

  addOutput(metadata: runtime_v1.TensorMetadata): void {
    if (metadata instanceof TensorMetadataBuilder) {
      this.meta.outputs.push(metadata.meta);
    }
  }
}

class ArgumentMetadataBuilder implements runtime_v1.ArgumentMetadata {
  constructor(public meta: ArgumentMetadata) {}

  setDescription(description: string): void {
    this.meta.description = description;
  }

  setDefaultValue(defaultValue: string): void {
    this.meta.defaultValue = defaultValue;
  }

  addHint(hint: runtime_v1.ArgumentHint): void {
    if (isArgumentHint(hint)) {
      this.meta.hints.push(hint);
    }
  }
}

class TensorMetadataBuilder implements runtime_v1.TensorMetadata {
  constructor(public meta: TensorMetadata) {}

  setDescription(description: string): void {
    this.meta.description = description;
  }

  addHint(hint: runtime_v1.TensorHint): void {
    if (isTensorHint(hint)) {
      this.meta.hints.push(hint);
    }
  }
}

function isArgumentHint(value?: any): value is ArgumentHint {
  const types: Array<ArgumentHint["type"]> = [
    "non-negative-number",
    "number-in-range",
    "string-enum",
    "supported-argument-type",
  ];

  return types.includes(value?.type);
}

function isTensorHint(value?: any): value is TensorHint {
  const types: Array<TensorHint["type"]> = ["media-hint", "supported-shapes"];

  return types.includes(value?.type);
}

export class GraphContext implements runtime_v1.GraphContext {
  inputs: TensorDescriptor[] = [];
  outputs: TensorDescriptor[] = [];

  constructor(private args: Record<string, string>) {}

  getArgument(name: string): string | null {
    if (name in this.args) {
      return this.args[name];
    } else {
      return null;
    }
  }

  addInputTensor(
    name: string,
    elementType: runtime_v1.ElementType,
    dimensions: runtime_v1.Dimensions
  ): void {
    this.inputs.push({ name, elementType, dimensions });
  }

  addOutputTensor(
    name: string,
    elementType: runtime_v1.ElementType,
    dimensions: runtime_v1.Dimensions
  ): void {
    this.outputs.push({ name, elementType, dimensions });
  }
}

export class KernelContext implements runtime_v1.KernelContext {
  public outputs: Record<string, runtime_v1.Tensor> = {};

  constructor(
    private args: Record<string, string>,
    private inputs: Record<string, runtime_v1.Tensor>
  ) {}

  getArgument(name: string): string | null {
    if (name in this.args) {
      return this.args[name];
    } else {
      return null;
    }
  }

  getInputTensor(name: string): runtime_v1.Tensor | null {
    if (name in this.inputs) {
      return this.inputs[name];
    } else {
      return null;
    }
  }

  setOutputTensor(name: string, tensor: runtime_v1.Tensor): void {
    this.outputs[name] = tensor;
  }

  getGlobalInput(name: string): runtime_v1.Tensor | null {
    throw new Error("Method not implemented.");
  }

  setGlobalOutput(name: string, tensor: runtime_v1.Tensor): void {
    throw new Error("Method not implemented.");
  }
}

export { ProcBlock } from "./ProcBlock";

import { runtime_v1 } from "@hotg-ai/rune-wit-files";

export type Metadata = {
  name: string;
  version: string;
  description?: string;
  repository?: string;
  homepage?: string;
  tags: string[];
  arguments: ArgumentMetadata[];
  inputs: TensorMetadata[];
  outputs: TensorMetadata[];
};

/**
 * Information about a tensor's name and constraints about its general shape.
 */
export type TensorDescriptor = {
  name: string;
  elementType: runtime_v1.ElementType;
  dimensions: runtime_v1.Dimensions;
};

export type TensorMetadata = {
  name: string;
  description?: string;
  hints: TensorHint[];
};

export type ArgumentMetadata = {
  name: string;
  description?: string;
  defaultValue?: string;
  hints: ArgumentHint[];
};

export type NumberInRange = {
  type: "number-in-range";
  min: string;
  max: string;
};

export type StringEnum = {
  type: "string-enum";
  possibleValues: string[];
};

export type NonNegativeNumber = {
  type: "non-negative-number";
};

export type SupportedArgumentType = {
  type: "supported-argument-type";
  argumentType: runtime_v1.ArgumentType;
};

export type ArgumentHint =
  | NumberInRange
  | StringEnum
  | NonNegativeNumber
  | SupportedArgumentType;

export type MediaHint = {
  type: "media-hint";
  media: "image" | "audio";
};

export type SupportedShapes = {
  type: "supported-shapes";
  supportedElementTypes: runtime_v1.ElementType[];
  dimensions: runtime_v1.Dimensions;
};

export type TensorHint = MediaHint | SupportedShapes;

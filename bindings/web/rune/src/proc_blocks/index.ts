export { ProcBlock } from "./ProcBlock";

import { runtime_v1 } from "@hotg-ai/rune-wit-files";

/**
 * Proc-block metadata.
 */
export type Metadata = {
  /**
   * The proc-block's human-friendly name.
   */
  name: string;
  /**
   * A semver-compliant version number.
   */
  version: string;
  /**
   * A long-form description of what this proc-block does, formatted as markdown.
   */
  description?: string;
  /**
   * A link to the proc-block's source code.
   */
  repository?: string;
  /**
   * A link to some web page associated with the proc-block.
   */
  homepage?: string;
  /**
   * Arbitrary tags that can be used for filtering and searching.
   */
  tags: string[];
  /**
   * Arguments this proc-block accepts.
   */
  arguments: ArgumentMetadata[];
  /**
   * The tensors this proc-block will expect as inputs.
   */
  inputs: TensorMetadata[];
  /**
   * The tensors this proc-block will produce as outputs.
   */
  outputs: TensorMetadata[];
};

/**
 * Information about a tensor's name and constraints about its general shape.
 */
export type TensorDescriptor = {
  /**
   * The name associated with this tensor.
   */
  name: string;
  /**
   * The type of elements this tensor will contain.
   */
  elementType: runtime_v1.ElementType;
  /**
   * Constraints on the tensor's dimensions (a 2D tensor with fixed dimensions,
   * a 1D tensor of arbitrary length, a tensor that can have any number of
   * dimensions it wants, etc.).
   */
  dimensions: runtime_v1.Dimensions;
};

/**
 * Metadata about a particular tensor.
 */
export type TensorMetadata = {
  /**
   * The name used by the proc-block when referring to this tensor.
   */
  name: string;
  /**
   * A long-form description of this tensor, formatted as markdown.
   */
  description?: string;
  hints: TensorHint[];
};

/**
 * Metadata around a proc-block argument.
 */
export type ArgumentMetadata = {
  /**
   * The name the proc-block will expect to find.
   */
  name: string;
  /**
   * A long-form description of what this argument does, formatted as markdown.
   */
  description?: string;
  /**
   * The value used by this if this argument isn't provided.
   */
  defaultValue?: string;
  /**
   * Arbitrary hints that can be used to understand more about the argument.
   */
  hints: ArgumentHint[];
};

/**
 * The argument has a numeric value within a particular range.
 */
export type NumberInRange = {
  type: "number-in-range";
  min: string;
  max: string;
};

/**
 * The argument should have one of the values within a set of possible values.
 */
export type StringEnum = {
  type: "string-enum";
  possibleValues: string[];
};

/**
 * The argument is a non-negative number.
 */
export type NonNegativeNumber = {
  type: "non-negative-number";
};

/**
 * The "type" of argument this may take.
 *
 * You can use this as a suggestion when trying to choose which widget would be
 * most appropriate when a user is inputting the argument value (e.g. you might
 * want to use a <textarea> when setting a "LongString").
 */
export type SupportedArgumentType = {
  type: "supported-argument-type";
  argumentType: runtime_v1.ArgumentType;
};

/**
 * A hint suggesting how an argument should be interpreted.
 */
export type ArgumentHint =
  | NumberInRange
  | StringEnum
  | NonNegativeNumber
  | SupportedArgumentType;

/**
 * How the tensor should be interpreted when it is presented to the user.
 */
export type MediaHint = {
  type: "media-hint";
  media: "image" | "audio";
};

/**
 * @deprecated Prefer calling the proc-block's graph() function.
 */
export type SupportedShapes = {
  type: "supported-shapes";
  supportedElementTypes: runtime_v1.ElementType[];
  dimensions: runtime_v1.Dimensions;
};

/**
 * Hints used when presenting tensors to the user.
 */
export type TensorHint = MediaHint | SupportedShapes;

/**
 * The tensors a proc-block will use as inputs and outputs.
 */
export type Tensors = {
  inputs: TensorDescriptor[];
  outputs: TensorDescriptor[];
};

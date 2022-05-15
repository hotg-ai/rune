/* tslint:disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

/**
 * The top level Runefile type.
 */
export type Document = DocumentV1;
/**
 *
 * A specification for finding a dependency.
 *
 * The full syntax is `base@version#sub_path` where
 *
 * - `base` is a URL or the name of a repository on GitHub (e.g. `hotg-ai/rune`
 *   or `https://github.com/hotg-ai/rune`)
 * - `version` is an optional field specifying the version (e.g. as a git tag)
 * - `sub_path` is an optional field which is useful when pointing to
 *   repositories with multiple relevant items because it lets you specify
 *   which directory the specified item is in.
 *
 */
export type Path = string;
/**
 * A stage in the Rune's pipeline.
 */
export type Stage = ModelStage | ProcBlockStage | CapabilityStage | OutStage;
/**
 * Something that could be either a reference to a resource (`$resource`) or a plain string (`./path`).
 */
export type Argument = ResourceName | number;
/**
 *
 * A reference to some [`ResourceDeclaration`]. It typically looks like
 * `$RESOURCE_NAME`.
 *
 */
export type ResourceName = string;
/**
 *
 * The name of a tensor.
 *
 * Typically something like "stage", or "stage.2" if the stage has multiple outputs.
 *
 */
export type Input = string;
/**
 * How the resource should be treated inside the Rune.
 */
export type ResourceType = "string" | "binary";

/**
 * Version 1 of the `Runefile.yml` format.
 */
export interface DocumentV1 {
  /**
   * The base image that defines the interface between a Rune and its runtime.
   *
   * This should always be `"runicos/base"`.
   */
  image: Path;
  /**
   * The various stages in the Runefile's pipeline.
   */
  pipeline: {
    [k: string]: Stage;
  };
  /**
   * Any resources that can be accessed by pipeline stages.
   */
  resources?: {
    [k: string]: ResourceDeclaration;
  };
  /**
   * The version number. Must always be `"1"`.
   */
  version: number;
  [k: string]: unknown;
}
/**
 * A ML model which will be executed by the runtime.
 */
export interface ModelStage {
  args?: {
    [k: string]: Argument;
  };
  /**
   * Tensors to use as input to this model.
   */
  inputs?: Input[];
  /**
   * The model to use, or a resource which specifies the model to use.
   */
  model: ResourceName;
  /**
   * The tensors that this model outputs.
   */
  outputs?: Type[];
  [k: string]: unknown;
}
/**
 * The element type and dimensions for a particular tensor.
 */
export interface Type {
  dimensions?: number[];
  type: string;
  [k: string]: unknown;
}
/**
 * A stage which executes a procedural block.
 */
export interface ProcBlockStage {
  args?: {
    [k: string]: Argument;
  };
  inputs?: Input[];
  outputs?: Type[];
  /**
   * A [`Path`] that Rune can use to locate the proc block.
   */
  "proc-block": string;
  [k: string]: unknown;
}
/**
 * A stage which reads inputs from the runtime.
 */
export interface CapabilityStage {
  args?: {
    [k: string]: Argument;
  };
  /**
   * What type of capability to use ("IMAGE", "SOUND", etc.).
   */
  capability: string;
  outputs?: Type[];
  [k: string]: unknown;
}
/**
 * A stage which passes outputs back to the runtime.
 */
export interface OutStage {
  args?: {
    [k: string]: Argument;
  };
  inputs?: Input[];
  /**
   * The type of output (e.g. "SERIAL").
   */
  out: string;
  [k: string]: unknown;
}
/**
 * The declaration for a resource, typically something like a wordlist or environment variable.
 */
export interface ResourceDeclaration {
  /**
   * A resource who's default value is specified inline.
   */
  inline?: string | null;
  /**
   * A resource who's default value is meant to be loaded from a file.
   */
  path?: string | null;
  type?: ResourceType & string;
}

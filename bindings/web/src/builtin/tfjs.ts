import * as tf from "@tensorflow/tfjs";
import { unzip } from 'unzipit';
import { concatenateArrayBuffers, getModelArtifactsForJSON } from '@tensorflow/tfjs-core/dist/io/io_utils';
import { IOHandler, ModelArtifacts, ModelJSON, SaveResult, WeightsManifestConfig, WeightsManifestEntry } from '@tensorflow/tfjs-core/dist/io/types';
import { TensorFlowModel } from "./TensorFlowModel";

/**
 * Load a TensorFlow model from a tf.js model that has been collected into a
 * single zip archive.
 */
export async function loadTensorFlowJS(buffer: ArrayBuffer): Promise<TensorFlowModel> {
  const { entries } = await unzip(buffer);

  if (!("model.json" in entries)) {
    throw new MissingModelJsonError(Object.keys(entries));
  }

  const modelJsonEntry = entries["model.json"];
  delete entries["model.json"];

  const modelJson = modelJsonEntry.json();

  if (!isModelJSON(modelJson)) {
    throw new Error(`The "model.json" file is malformed`);
  }

  const weights: Record<string, ArrayBuffer> = {};

  for (const [key, value] of Object.entries(entries)) {
    weights[key] = await value.arrayBuffer();
  }

  const io = new ShardedModel(modelJson, weights);
  const model = await tf.loadGraphModel(io);

  return new TensorFlowModel(model);
}

/**
 * An `IOHandler` loosely based on [the HTTPRequest type][HTTPRequest] from
 * `@tensorflow/tfjs-core` (Apache 2.0 license).
 *
 * [HTTPRequest]: https://github.com/tensorflow/tfjs/blob/d95abc0028365da71beaca060064e7b4ad7a1f86/tfjs-core/src/io/http.ts
 */
export class ShardedModel implements IOHandler {
  private readonly json: ModelJSON;
  private readonly weights: Record<string, ArrayBuffer | undefined>;

  constructor(json: ModelJSON, weights: Record<string, ArrayBuffer | undefined>) {
    this.json = json;
    this.weights = weights;
  }

  async save(): Promise<SaveResult> {
    throw new Error("Model saving is not supported");
  }

  /**
   * Load model frombuffers.
   *
   * @returns The loaded model artifacts (if loading succeeds).
   */
  async load(): Promise<ModelArtifacts> {
    return getModelArtifactsForJSON(
      this.json, manifest => this.loadWeights(manifest));
  }

  private async loadWeights(weightsManifest: WeightsManifestConfig):
    Promise<[WeightsManifestEntry[], ArrayBuffer]> {

    const weightSpecs = [];
    const buffers: ArrayBuffer[] = [];

    for (const entry of weightsManifest) {
      weightSpecs.push(...entry.weights);

      for (const path of entry.paths) {
        const weight = this.weights[path];
        if (!weight) {
          throw new Error(`Unable to find the "${path}" weight`);
        }
        buffers.push(weight);
      }
    }

    return [weightSpecs, concatenateArrayBuffers(buffers)];
  }
}

class MissingModelJsonError extends Error {
  constructor(public readonly entries: string[]) {
    super(`Unable to find "model.json" in the zip archive`);
  }
}

function isModelJSON(item?: any): item is ModelJSON {
  return item
    && typeof item.modelTopology === "object"
    && typeof item.weightsManifest === "object";
}

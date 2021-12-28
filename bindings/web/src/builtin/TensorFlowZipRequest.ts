/**
 * @license
 * Copyright 2018 Google LLC. All Rights Reserved.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * =============================================================================
 */

/**
 * IOHandler implementations based on HTTP requests in the web browser.
 *
 * Uses [`fetch`](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API).
 */


import { assert } from '@tensorflow/tfjs-core/dist/util'//'../util';
import { concatenateArrayBuffers, getModelArtifactsForJSON, getModelArtifactsInfoForJSON, getModelJSONForModelArtifacts } from '@tensorflow/tfjs-core/dist/io/io_utils';
import { IORouter, IORouterRegistry } from '@tensorflow/tfjs-core/dist/io/router_registry';
import { IOHandler, LoadOptions, ModelArtifacts, ModelJSON, OnProgressCallback, SaveResult, WeightsManifestConfig, WeightsManifestEntry } from '@tensorflow/tfjs-core/dist/io/types';
import { loadWeightsAsArrayBuffer } from '@tensorflow/tfjs-core/dist/io/weights_loader';

const OCTET_STREAM_MIME_TYPE = 'application/octet-stream';
const JSON_TYPE = 'application/json';
export class TensorFlowZipRequest implements IOHandler {

  private readonly json: ArrayBuffer;
  private readonly weights: Map<string, ArrayBuffer>;

  constructor(json: ArrayBuffer, weights: Map<string, ArrayBuffer>) {
    this.json = json;
    this.weights = weights;
  }

  async save(modelArtifacts: ModelArtifacts): Promise<SaveResult> {
    throw new Error(
      'TensorFlowZipRequest does not support saving model topology ');
  }

  /**
   * Load model frombuffers.
   *
   * @returns The loaded model artifacts (if loading succeeds).
   */
  async load(): Promise<ModelArtifacts> {
    let modelJSON: ModelJSON;
    try {
      const decoder = new TextDecoder("utf-8");
      let decoded = decoder.decode(this.json);
      modelJSON = JSON.parse(decoded);
    } catch (e) {
      let message = `Failed to parse model JSON`;
      throw new Error(message);
    }

    // We do not allow both modelTopology and weightsManifest to be missing.
    const modelTopology = modelJSON.modelTopology;
    const weightsManifest = modelJSON.weightsManifest;
    if (modelTopology == null && weightsManifest == null) {
      throw new Error(
        `The JSON from HTTP path contains neither model ` +
        `topology or manifest for weights.`);
    }

    return getModelArtifactsForJSON(
      modelJSON, (weightsManifest) => this.loadWeights(weightsManifest));
  }

  private async loadWeights(weightsManifest: WeightsManifestConfig):
    Promise<[WeightsManifestEntry[], ArrayBuffer]> {

    const weightSpecs = [];
    const buffers: ArrayBuffer[] = [];

    for (const entry of weightsManifest) {
      weightSpecs.push(...entry.weights);
      for (const path of entry.paths) {
        buffers.push(this.weights.get(path) ?? new ArrayBuffer(0))
      }
    }
    if (buffers.length == 0) {
      throw new Error(
        `No weight data found `);
    }

    return [weightSpecs, concatenateArrayBuffers(Array.from(buffers))];
  }
}

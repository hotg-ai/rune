import { loadLayersModel } from "@tensorflow/tfjs-layers";
import { InferenceModel } from "@tensorflow/tfjs-core";
import { unzip, ZipEntry } from "unzipit";
import type {
  IOHandler,
  ModelArtifacts,
  ModelJSON,
  SaveResult,
  WeightsManifestConfig,
  WeightsManifestEntry,
} from "@tensorflow/tfjs-core/dist/io/types";
import { TensorFlowModel } from "./TensorFlowModel";
import { loadGraphModel } from "@tensorflow/tfjs-converter";

/**
 * Load a TensorFlow model from a tf.js model that has been collected into a
 * single zip archive.
 */
export async function loadTensorFlowJS(
  buffer: ArrayBuffer
): Promise<TensorFlowModel> {
  const { entries } = await unzip(buffer);

  const [modelJson, weights] = await extractModelAndWeights(
    Object.values(entries)
  );

  const io = new ShardedModel(modelJson, weights);
  const model = await loadModel(modelJson.format, io);

  return new TensorFlowModel(model);
}

async function extractModelAndWeights(
  entries: ZipEntry[]
): Promise<[ModelJSON, Record<string, ArrayBuffer>]> {
  // first we need to find the "model.json" file
  const modelJsonEntry = entries.find(
    (k) => k.name.split("/").pop() === "model.json"
  );

  if (!modelJsonEntry) {
    throw new MissingModelJsonError(entries.map((e) => e.name));
  }

  const model = await modelJsonEntry.json();

  if (!isModelJSON(model)) {
    throw new Error(`The "model.json" file is malformed`);
  }

  // Note: Sometimes the model might be in a nested directory.
  //
  // The "model.json" refers to each model file relative to that directory, so
  // want to only take weights in that directory and update their filenames
  // appropriately.
  const directory = modelJsonEntry.name.replace(/model\.json$/, "");

  const pendingWeights = entries
    .filter((e) => !e.isDirectory && e.name.startsWith(directory))
    .map(async (e): Promise<[string, ArrayBuffer]> => {
      const name = e.name.replace(directory, "");
      const buffer = await e.arrayBuffer();
      return [name, buffer];
    });

  const weights = Object.fromEntries(await Promise.all(pendingWeights));

  return [model, weights];
}

async function loadModel(
  format: string | undefined,
  io: ShardedModel
): Promise<InferenceModel> {
  switch (format) {
    case "graph-model":
      return await loadGraphModel(io);

    case "layers-model":
      // Note: LayersModel actually implements the InferenceModel interface if
      // you look at the source code, but TypeScript complains because the
      // implementing methods have a slightly different signature.
      //
      // This should be fine, so let's just use a typecast and keep going.
      return (await loadLayersModel(io)) as InferenceModel;

    default:
      throw new Error(`Unknown format: "${format}"`);
  }
}

class MissingModelJsonError extends Error {
  constructor(public readonly entries: string[]) {
    super(`Unable to find "model.json" in the zip archive`);
  }
}

function isModelJSON(item?: any): item is ModelJSON {
  return (
    item &&
    typeof item.modelTopology === "object" &&
    typeof item.weightsManifest === "object"
  );
}

/**
 * An `IOHandler` loosely based on [the HTTPRequest type][HTTPRequest] from
 * `@tensorflow/tfjs-core` (Apache 2.0 license).
 *
 * [HTTPRequest]: https://github.com/tensorflow/tfjs/blob/d95abc0028365da71beaca060064e7b4ad7a1f86/tfjs-core/src/io/http.ts
 */
class ShardedModel implements IOHandler {
  private readonly json: ModelJSON;
  private readonly weights: Record<string, ArrayBuffer | undefined>;

  constructor(
    json: ModelJSON,
    weights: Record<string, ArrayBuffer | undefined>
  ) {
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
    return getModelArtifactsForJSON(this.json, (manifest) =>
      this.loadWeights(manifest)
    );
  }

  private async loadWeights(
    weightsManifest: WeightsManifestConfig
  ): Promise<[WeightsManifestEntry[], ArrayBuffer]> {
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

/*
 * HACK: The following functions have been copied straight from the
 * "io_util.ts" file in @tensorflow/tfjs-core to work around build issues where
 * Jest can't load the compiled javaScript in tfjs's dist directory because it
 * uses "import".
 *
 * See https://github.com/tensorflow/tfjs/blob/d95abc0028365da71beaca060064e7b4ad7a1f86/tfjs-core/src/io/io_utils.ts
 * (Apache-2.0 license)
 */

/**
 * Create `ModelArtifacts` from a JSON file.
 *
 * @param modelJSON Object containing the parsed JSON of `model.json`
 * @param loadWeights Function that takes the JSON file's weights manifest,
 *     reads weights from the listed path(s), and returns a Promise of the
 *     weight manifest entries along with the weights data.
 * @returns A Promise of the `ModelArtifacts`, as described by the JSON file.
 */
async function getModelArtifactsForJSON(
  modelJSON: ModelJSON,
  loadWeights: (
    weightsManifest: WeightsManifestConfig
  ) => Promise<
    [/* weightSpecs */ WeightsManifestEntry[], /* weightData */ ArrayBuffer]
  >
): Promise<ModelArtifacts> {
  const modelArtifacts: ModelArtifacts = {
    modelTopology: modelJSON.modelTopology,
    format: modelJSON.format,
    generatedBy: modelJSON.generatedBy,
    convertedBy: modelJSON.convertedBy,
  };

  if (modelJSON.trainingConfig != null) {
    modelArtifacts.trainingConfig = modelJSON.trainingConfig;
  }
  if (modelJSON.weightsManifest != null) {
    const [weightSpecs, weightData] = await loadWeights(
      modelJSON.weightsManifest
    );
    modelArtifacts.weightSpecs = weightSpecs;
    modelArtifacts.weightData = weightData;
  }
  if (modelJSON.userDefinedMetadata != null) {
    modelArtifacts.userDefinedMetadata = modelJSON.userDefinedMetadata;
  }

  return modelArtifacts;
}

/**
 * Concatenate a number of ArrayBuffers into one.
 *
 * @param buffers A number of array buffers to concatenate.
 * @returns Result of concatenating `buffers` in order.
 */
function concatenateArrayBuffers(buffers: ArrayBuffer[]): ArrayBuffer {
  if (buffers.length === 1) {
    return buffers[0];
  }

  let totalByteLength = 0;
  buffers.forEach((buffer: ArrayBuffer) => {
    totalByteLength += buffer.byteLength;
  });

  const temp = new Uint8Array(totalByteLength);
  let offset = 0;
  buffers.forEach((buffer: ArrayBuffer) => {
    temp.set(new Uint8Array(buffer), offset);
    offset += buffer.byteLength;
  });
  return temp.buffer;
}

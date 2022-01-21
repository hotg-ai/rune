import fs from "fs";
import path from "path";

const projectRoot = path.join(__dirname, "../../../../..");

export function layersModel(): ArrayBuffer {
  const filename = path.join(__dirname, "sine-wave.tfjs-layers.zip");
  return fs.readFileSync(filename);
}

export function graphModel(): ArrayBuffer {
  const filename = path.join(__dirname, "sine-wave.tfjs-graph.zip");
  return fs.readFileSync(filename);
}

export function tfliteModel(): ArrayBuffer {
  const filename = path.join(
    projectRoot,
    "examples",
    "sine",
    "sinemodel.tflite"
  );
  return fs.readFileSync(filename);
}

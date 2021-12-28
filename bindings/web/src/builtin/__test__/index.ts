import fs from "fs";
import path from "path";

const projectRoot = path.join(__dirname, "../../../../");

/**
 * Load a tf.js model which was generated as part of the
 * "directories-are-zipped-when-used-for-models" integration test.
 */
export function tfjsModel(): ArrayBuffer {
    const filename = path.join(__dirname, "sine-wave.tfjs.zip");
    return fs.readFileSync(filename);
}

export function tfliteModel(): ArrayBuffer {
    const filename = path.join(projectRoot, "examples", "sine", "sinemodel.tflite");
    return fs.readFileSync(filename);
}

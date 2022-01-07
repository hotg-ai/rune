// FIXME: this test can be uncommented when @tensorflow/tfjs-tflite is usable
//  outside of a web browser.
// See https://github.com/tensorflow/tfjs/issues/5532

// import { loadTensorFlowLite } from "./tflite";
import { tfliteModel } from "./__test__";

describe.skip("TensorFlowModel", () => {
    it("can load a tflite model", async () => {
        const buffer = tfliteModel();

        // const model = await loadTensorFlowLite(buffer);

        // console.log(model.inputs, model.outputs);
        // expect(model.inputs).toEqual("...");
        // expect(model.outputs).toEqual("...");
    })
});


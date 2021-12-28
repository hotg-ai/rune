import { loadTensorFlowLite } from "./tflite";
import { tfliteModel } from "./__test__";

describe("TensorFlowModel", () => {
    it("can load a tflite model", async () => {
        const buffer = tfliteModel();

        const model = await loadTensorFlowLite(buffer);

        expect(model.inputs).toEqual({});
        expect(model.outputs).toEqual({});
    })
});


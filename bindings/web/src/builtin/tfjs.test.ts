import Shape from "../Shape";
import { loadTensorFlowJS } from "./tfjs";
import { tfjsModel } from "./__test__";

describe("TensorFlowModel", () => {
    it("can load a tf.js model", async () => {
        const buffer = tfjsModel();

        const model = await loadTensorFlowJS(buffer);

        expect(model.inputs).toEqual([new Shape("float32", [1])]);
        expect(model.outputs).toEqual([new Shape("float32", [1])]);
    })
});

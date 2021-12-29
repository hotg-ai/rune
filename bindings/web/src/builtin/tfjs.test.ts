import Shape from "../Shape";
import { loadTensorFlowJS } from "./tfjs";
import { graphModel, layersModel } from "./__test__";

describe("TensorFlowModel", () => {
    it("can load a tf.js layers model", async () => {
        const buffer = layersModel();

        const model = await loadTensorFlowJS(buffer);

        expect(model.inputs).toEqual([new Shape("float32", [1])]);
        expect(model.outputs).toEqual([new Shape("float32", [1])]);
    });

    it("can load a tf.js graph model", async () => {
        const buffer = graphModel();

        const model = await loadTensorFlowJS(buffer);

        expect(model.inputs).toEqual([new Shape("float32", [1])]);
        // Note: the graph model's output tensor actually looks something
        // like { name: "Identity", shape: undefined, dtype: undefined }, so
        // we use the fallback values.
        expect(model.outputs).toEqual([new Shape("unknown", [])]);
    });
});

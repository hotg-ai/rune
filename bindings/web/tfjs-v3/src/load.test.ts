import { Shape } from "@hotg-ai/rune";
import { loadTensorFlowJS } from "./load";
import { graphModel, layersModel } from "./__test__";

describe("TensorFlowModel", () => {
    it("can load a tf.js layers model", async () => {
        const buffer = layersModel();

        const model = await loadTensorFlowJS(buffer);

        expect(model.inputs).toEqual([new Shape("float32", [1])]);
        expect(model.outputs).toEqual([new Shape("float32", [1])]);

        // According to our training notebook, sine(3.0) = 0.1255441
        const input = new Float32Array([3.0]);
        const inputs = [new Uint8Array(input.buffer)];
        const output = new Float32Array(1);
        const outputs = [new Uint8Array(output.buffer)];
        const shape = [Shape.parse("f32[1]")];
        model.transform(inputs, shape, outputs, shape);
        expect(output[0]).toBeCloseTo(0.13199206);
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

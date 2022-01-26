import { Shape } from "@hotg-ai/rune";
import { loadTensorFlowJS } from "./load";
import { graphModel, layersModel } from "./__test__";

describe("TensorFlowModel", () => {
  it("can load a tf.js layers model", async () => {
    const buffer = layersModel();

    const model = await loadTensorFlowJS(buffer);

    expect(model.inputs).toEqual([new Shape("float32", [1])]);
    expect(model.outputs).toEqual([new Shape("float32", [1])]);


    // Note: we use part of a larger array because there have been bugs
    // where don't create typed arrays correctly with the offset and length.
    // See https://github.com/hotg-ai/rune/pull/402
    const backingBuffer = new Float32Array([0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
    const inputs = [new Uint8Array(backingBuffer.buffer, Float32Array.BYTES_PER_ELEMENT * 3, Float32Array.BYTES_PER_ELEMENT)];
    const output = new Float32Array(1);
    const outputs = [new Uint8Array(output.buffer)];
    const shape = [Shape.parse("f32[1, 1]")];
    model.transform(inputs, shape, outputs, shape);
    // According to our training notebook, sine(3.0) = 0.1255441.
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

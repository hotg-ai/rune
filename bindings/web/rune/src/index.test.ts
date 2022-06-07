import fs from "fs";
import path from "path";
import { Node, ElementType, Tensor, Rune } from ".";
import { Tensors } from "./proc_blocks";
import { floatTensor } from "./utils";
import { testLogger } from "./__test__";

describe("Integration Tests", () => {
  let logger = testLogger();

  const sine = new Uint8Array(
    fs.readFileSync(path.join(__dirname, "__fixtures__", "sine.zip"))
  );

  it("can load the sine Rune", async () => {
    const loader = new Rune();

    const runtime = await loader
      .withModelHandler("tensorflow-lite", async () => new DummySineModel())
      .withLogger(logger)
      .load(sine);

    runtime.setInput("rand", floatTensor([1]));

    await runtime.infer();

    expect(runtime.outputs).toEqual({
      serial: [floatTensor([Math.sin(1)])],
    });
  });
});

/**
 * A "model" that executes sine() against each element in the tensor it is
 * given.
 */
class DummySineModel implements Node {
  async graph(): Promise<Tensors> {
    const tensor = {
      elementType: ElementType.F32,
      dimensions: {
        tag: "fixed",
        val: Uint32Array.from([1, 1]),
      },
    } as const;

    return {
      inputs: [{ ...tensor, name: "input" }],
      outputs: [{ ...tensor, name: "output" }],
    };
  }

  async infer(
    inputs: Record<string, Tensor>,
    args: Record<string, string>
  ): Promise<Record<string, Tensor>> {
    const {
      input: {
        buffer: { buffer, byteLength, byteOffset },
        dimensions,
        elementType,
      },
    } = inputs;

    if (elementType != ElementType.F32) {
      throw new Error("Invalid element type");
    }

    const floats = new Float32Array(
      buffer,
      byteOffset,
      byteLength / Float32Array.BYTES_PER_ELEMENT
    );

    const result = floats.map(Math.sin);

    return {
      output: {
        elementType: ElementType.F32,
        dimensions,
        buffer: new Uint8Array(
          result.buffer,
          result.byteOffset,
          result.byteLength
        ),
      },
    };
  }
}

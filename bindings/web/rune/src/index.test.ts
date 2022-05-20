import fs from "fs";
import { load } from "js-yaml";
import path from "path";
import { RuneLoader, Node, ElementType, Tensor } from ".";
import { Tensors } from "./proc_blocks";

describe("Integration Tests", () => {
  const sine = new Uint8Array(
    fs.readFileSync(path.join(__dirname, "__fixtures__", "sine.zip"))
  );

  it("can load the sine Rune", async () => {
    const loader = new RuneLoader();

    const runtime = await loader
      .withModelHandler("tensorflow-lite", async () => new DummySineModel())
      .load(sine);

    for (const name of Object.keys(runtime.inputs)) {
      runtime.setInput(name, {
        buffer: new Uint8Array(),
        dimensions: new Uint32Array(),
        elementType: ElementType.F32,
      });
    }

    await runtime.infer();

    console.log(runtime);
    expect(false).toBeTruthy();
  });
});

class DummySineModel implements Node {
  async graph(): Promise<Tensors> {
    const tensor = {
      elementType: ElementType.F32,
      dimensions: {
        tag: "fixed",
        val: Uint32Array.from([1]),
      },
    } as const;

    return {
      inputs: [{ ...tensor, name: "input" }],
      outputs: [{ ...tensor, name: "output" }],
    };
  }

  infer(
    inputs: Record<string, Tensor>,
    args: Record<string, string>
  ): Promise<Record<string, Tensor>> {
    throw new Error("Method not implemented.");
  }
}

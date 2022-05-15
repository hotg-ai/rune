import fs from "fs";
import path from "path";
import { consoleLogger, RuneLoader, Node, ElementType, Tensor } from ".";
import { Tensors } from "./proc_blocks";

describe("Integration Tests", () => {
  const sine = new Uint8Array(
    fs.readFileSync(path.join(__dirname, "__fixtures__", "sine.zip"))
  );

  it("can load the sine Rune", async () => {
    const loader = new RuneLoader();

    const runtime = await loader
      .withModelHandler("tensorflow-lite", async () => new DummySineModel())
      .withLogger(consoleLogger)
      .load(sine);

    runtime.infer();
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

import yaml from "js-yaml";
import { TensorDescriptor, Tensors } from "./proc_blocks";
import { DocumentV1 } from "./Runefile";
import { Node } from ".";
import { Runtime, create } from "./Runtime";
import { ElementType, Tensor } from ".";
import { floatTensor } from "./utils";

describe("Runtime2", () => {
  const src = `
      version: 1
      image: runicos/base
      pipeline:
        rand:
          capability: RAW
          outputs:
            - type: F32
              dimensions:
                - 1
                - 1
          args:
            length: "4"
        mod360:
          proc-block: proc_blocks/mod360
          inputs:
            - rand
          outputs:
            - type: F32
              dimensions:
                - 1
                - 1
          args:
            modulus: "360"
        sine:
          model: models/sine
          inputs:
            - mod360
          outputs:
            - type: F32
              dimensions:
                - 1
                - 1
        serial:
          out: serial
          inputs:
            - sine
      resources: {}`;
  const runefile = yaml.load(src) as DocumentV1;

  const f32_1x1 = {
    elementType: ElementType.F32,
    dimensions: { tag: "fixed", val: Uint32Array.from([1]) },
  } as const;

  const rand = dummyProcBlock([], [{ name: "output", ...f32_1x1 }], {
    output: floatTensor([1]),
  });
  const mod360 = dummyProcBlock(
    [{ name: "input", ...f32_1x1 }],
    [{ name: "output", ...f32_1x1 }],
    { output: floatTensor([2]) }
  );

  const sine = dummyNode(
    [{ name: "input", ...f32_1x1 }],
    [{ name: "output", ...f32_1x1 }],
    { output: floatTensor([3]) }
  );

  it("can run the sine Rune", async () => {
    const procBlocks = { rand, mod360 };
    const models = { sine };

    const runtime: Runtime = create(runefile, procBlocks, models);

    runtime.setInput("rand", floatTensor([0]));

    await runtime.infer();

    const outputs = runtime.outputTensors;
    expect(outputs).toMatchObject({
      serial: [floatTensor([3])],
    });
  });
});

function dummyProcBlock(
  inputs: TensorDescriptor[],
  outputs: TensorDescriptor[],
  results: Record<string, Tensor>
) {
  return {
    graph: (): Tensors => {
      return { inputs, outputs };
    },
    evaluate: () => results,
  };
}

function dummyNode(
  inputs: TensorDescriptor[],
  outputs: TensorDescriptor[],
  results: Record<string, Tensor>
): Node {
  return {
    graph: async (): Promise<Tensors> => {
      return { inputs, outputs };
    },
    infer: () => Promise.resolve(results),
  };
}

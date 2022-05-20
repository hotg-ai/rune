import { runtime_v1 } from "@hotg-ai/rune-wit-files";
import yaml from "js-yaml";
import { Node } from ".";
import { ElementType, Tensor } from "..";
import { consoleLogger, Logger, StructuredLogger } from "../logging";
import { TensorDescriptor, Tensors } from "../proc_blocks";
import { DocumentV1 } from "../Runefile";
import { determinePipeline, Pipeline } from "./pipeline";

describe("pipeline", () => {
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

  it.skip("can determine the pipeline for sine", async () => {
    const logger = { log: consoleLogger, isEnabled: () => true };
    const procBlocks = {
      rand: dummyProcBlock([], [{ name: "output", ...f32_1x1 }]),
      mod360: dummyProcBlock(
        [{ name: "input", ...f32_1x1 }],
        [{ name: "output", ...f32_1x1 }]
      ),
    };
    const models = {
      sine: dummyNode(
        [{ name: "input", ...f32_1x1 }],
        [{ name: "output", ...f32_1x1 }]
      ),
    };

    const pipeline = await determinePipeline(
      runefile,
      procBlocks,
      models,
      logger
    );

    // Note: we can't compare nodes for equality because they are objects
    const { nodes, ...rest } = pipeline;

    const expected: Omit<Pipeline, "nodes"> = {
      nodeInfo: {
        "0": {
          name: "rand",
          inputs: {
            "0": 1,
          },
          outputs: {
            "0": 2,
          },
          args: { length: "4" },
        },
        "1": {
          name: "mod360",
          inputs: {
            "0": 3,
          },
          outputs: {
            "0": 4,
          },
          args: {},
        },
      },
      evaluationOrder: ["42"],
      inputs: ["42"],
      tensors: {
        42: {
          elementType: ElementType.F32,
          dimensions: {
            tag: "fixed",
            val: Uint32Array.from([1, 1]),
          },
        },
      },
      outputTensors: {},
    };
    expect(rest).toMatchObject(expected);
  });
});

function dummyProcBlock(
  inputs: TensorDescriptor[],
  outputs: TensorDescriptor[]
) {
  return {
    graph: (): Tensors => {
      return { inputs, outputs };
    },
    evaluate: () => {
      throw new Error();
    },
  };
}

function dummyNode(
  inputs: TensorDescriptor[],
  outputs: TensorDescriptor[]
): Node {
  return {
    graph: async (): Promise<Tensors> => {
      return { inputs, outputs };
    },
    infer: () => {
      throw new Error();
    },
  };
}

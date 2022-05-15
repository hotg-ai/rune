import yaml from "js-yaml";
import { consoleLogger, Logger, StructuredLogger } from "../logging";
import { DocumentV1 } from "../Runefile";
import { determinePipeline } from "./pipeline";

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

  it("can determine the pipeline for sine", () => {
    const logger = { log: consoleLogger, isEnabled: () => true };

    const pipeline = determinePipeline(runefile, logger);

    // Note: we can't compare nodes for equality because they are objects
    const { nodes, ...rest } = pipeline;

    expect(rest).toMatchObject({
      nodeInfo: { 1: { name: "asf" } },
      evaluationOrder: [42],
      inputs: [42],
      tensors: {
        91: { name: "asdf" },
      },
    });
  });
});

import { consoleLogger, RuneLoader } from ".";

describe("Integration Tests", () => {
  it("can load the sine Rune", async () => {
    const loader = new RuneLoader();

    const runtime = await loader
      .withLogger(consoleLogger)
      .load(new ArrayBuffer(0));

    runtime.infer();
  });
});

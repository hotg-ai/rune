import { Imports, Runtime } from "rune";
import fs from "fs/promises";
import { join } from "path";

describe("Runtime", () => {
    it("can load the sine Rune", async () => {
        const sine = await fs.readFile(join(__dirname, "sine.rune"));
        const imports = new Imports();

        const runtime = await Runtime.load(sine, imports);

        runtime.call();
    });
});

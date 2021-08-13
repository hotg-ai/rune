const { Blob } = require("blob-polyfill");
global.Blob = Blob;

import child_process from "child_process";
import path from "path";
import fs from "fs";
import { Runtime } from "./Runtime";
import { Capability, Output } from ".";

const decoder = new TextDecoder("utf8");

describe.skip("Runtime", () => {
    const noopRune = buildExample("noop");

    it("can load the noop Rune", async () => {
        const calls: Uint8Array[] = [];
        const imports = {
            capabilities: {
                raw: () => new RawCapability(),
            },
            outputs: {
                serial: () => new SpyOutput(calls),
            },
        };

        const runtime = await Runtime.load(noopRune, imports);

        expect(runtime).not.toBeNull();
        expect(calls).toHaveLength(1);
        const output = decoder.decode(calls[0]);
        expect(JSON.parse(output)).toEqual({ asd: "TODO" });
    });
});

class RawCapability implements Capability {
    generate(dest: Uint8Array, id: number): void {
        throw new Error("Method not implemented.");
    }
}

class SpyOutput implements Output {
    received: Uint8Array[];
    constructor(received: Uint8Array[]) {
        this.received = received;
    }

    consume(data: Uint8Array): void {
        this.received.push(data);
    }
}

function buildExample(name: string): ArrayBuffer {
    const gitOutput = child_process.execSync("git rev-parse --show-toplevel");
    const repoRoot = decoder.decode(gitOutput).trim();

    const exampleDir = path.join(repoRoot, "examples", name);
    const runefile = path.join(exampleDir, "Runefile.yml");

    child_process.execSync(`cargo rune build ${runefile} --quiet`, {
        cwd: repoRoot,
        env: {
            RUST_LOG: "warning",
            ...process.env
        },
    });
    const rune = path.join(exampleDir, name + ".rune");

    return fs.readFileSync(rune);
}

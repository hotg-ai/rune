const { Blob } = require("blob-polyfill");
global.Blob = Blob;

import child_process from "child_process";
import path from "path";
import fs from "fs";
import { Runtime, Capability, Output } from "./Runtime";

const decoder = new TextDecoder("utf8");

describe("Runtime", () => {
    const noopRune = buildExample("noop");

    it("can load the noop Rune", async () => {
        const imports = {
            createCapability: () => new RawCapability(),
            createOutput: () => new SpyOutput([]),
            createModel: () => { throw new Error(); },
            log: (msg: any) => { },
        };

        const runtime = await Runtime.load(noopRune, imports);

        expect(runtime).not.toBeNull();
    });

    it("can run the noop Rune", async () => {
        const calls: Uint8Array[] = [];
        const imports = {
            createCapability: () => new RawCapability([
                1, 0, 0, 0,
                2, 0, 0, 0,
                3, 0, 0, 0,
                4, 0, 0, 0,
            ]),
            createOutput: () => new SpyOutput(calls),
            createModel: () => { throw new Error(); },
            log: (msg: any) => { },
        };
        const runtime = await Runtime.load(noopRune, imports);

        runtime.call();

        expect(calls).toHaveLength(1);
        const output = decoder.decode(calls[0]);
        expect(JSON.parse(output)).toEqual({
            channel: 1,
            dimensions: [4,],
            elements: [1, 2, 3, 4,],
            type_name: "i32",
        });
    });
});

class RawCapability implements Capability {
    data: Uint8Array = new Uint8Array();

    constructor(data?: number[]) {
        if (data) {
            this.data = Uint8Array.from(data);
        }
    }

    setParameter(name: string, value: number): void {
        throw new Error("Method not implemented.");
    }
    generate(dest: Uint8Array): void {
        dest.set(this.data);
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

    child_process.execSync(`cargo run --bin rune --quiet -- build ${runefile} --quiet --unstable --rune-repo-dir ${repoRoot}`, {
        cwd: repoRoot,
        env: {
            RUST_LOG: "warning",
            ...process.env
        },
    });
    const rune = path.join(exampleDir, name + ".rune");

    return fs.readFileSync(rune);
}

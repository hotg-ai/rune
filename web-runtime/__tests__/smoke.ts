import "jest";
import { Capability, Imports, Model, Output, Outputs } from "../src";
import Runtime from "../src/Runtime";
import runes from "../__mocks__/runes";

describe("Web Runtime", () => {
    it("should load a Rune", async () => {
        const sine = await runes.sine();
        const module = await WebAssembly.compile(sine);

        const got = await Runtime.load(module, trivialImports(), () => new DummyModel());

        expect(got).toBeDefined();
    });

    it("should invoke a Rune", async () => {
        const sine = await runes.sine();
        const module = await WebAssembly.compile(sine);
        const imports = trivialImports();
        const serial = imports.outputs.serial() as SerialOutput;
        const runtime = await Runtime.load(module, imports, () => new DummyModel());

        runtime.call();

        expect(serial.calls).toEqual([[0]]);
    });
});

function trivialImports(): Imports {
    const capabilities = {
        rand: () => new RandomCapability(),
    };
    const serial = new SerialOutput();
    const outputs = {
        serial: () => serial,
    };

    return { capabilities, outputs };
}

class RandomCapability implements Capability {
    generate(dest: Uint8Array): void {
        for (let i = 0; i < dest.length; i++) {
            dest[i] = Math.floor(Math.random() * 256);
        }
    }
    set(key: string, value: number): void {
        throw new Error("Method not implemented.");
    }
}

class SerialOutput implements Output {
    public calls: any[] = [];

    consume(data: Uint8Array): void {
        const utf8 = new TextDecoder();
        this.calls.push(JSON.parse(utf8.decode(data)));
    }

}

class DummyModel implements Model {
    transform(input: Uint8Array, output: Uint8Array): void {
    }
}

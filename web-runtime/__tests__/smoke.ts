import "jest";
import { Capabilities, Capability, Imports, loadRuntime, Model, Output, Outputs } from "../src";
import runes from "../__mocks__/runes";

describe("Web Runtime", () => {
    it("should load a Rune", async () => {
        const sine = await runes.sine();
        const module = await WebAssembly.compile(sine);

        const got = await loadRuntime(module, trivialImports());

        expect(got).toBeDefined();
    });
});

function trivialImports(): Imports {
    return {
        capabilities(): Capabilities {
            return {
                rand: () => new RandomCapability(),
            };
        },
        outputs(): Outputs {
            return {
                serial: () => new SerialOutput(),
            };
        },
        loadModel: () => new DummyModel(),
    }
}

class RandomCapability implements Capability {
    generate(dest: Uint8Array): void {
        throw new Error("Method not implemented.");
    }
    set(key: string, value: number): void {
        throw new Error("Method not implemented.");
    }
}

class SerialOutput implements Output {
    consume(data: Uint8Array): void {
        throw new Error("Method not implemented.");
    }

}

class DummyModel implements Model { }

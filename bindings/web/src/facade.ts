import { Tensor } from "@tensorflow/tfjs-core";
import { Capability, Imports, Model, Output, Runtime } from ".";

const Capabilities = {
    "rand": 1,
    "sound": 2,
    "accel": 3,
    "image": 4,
    "raw": 5,
}

export type CapabilityType = keyof typeof Capabilities;
type ModelConstructor = (model: ArrayBuffer) => Promise<Model>;
type Logger = (message: string) => void;

export type InputDescription = {
    type: CapabilityType,
    args: Partial<Record<string, number>>,
};
export type ReadInput = (input: InputDescription) => Tensor;

export class Builder {
    private modelHandlers: Partial<Record<string, ModelConstructor>> = {};
    private log: Logger = console.log;

    /**
     * Set a handler that will be called every time the Rune logs a message.
     */
    public onDebug(handler: Logger): this {
        this.log = handler;
        return this;
    }

    /**
     * Add support for a new type of model.
     * @param mimetype The "mimetype" that specifies which type of model being
     * handled.
     * @param constructor A constructor which will load the model.
     * @returns
     */
    public withModelHandler(mimetype: string, constructor: ModelConstructor): this {
        this.modelHandlers[mimetype] = constructor;

        return this;
    }

    public async build(rune: ArrayBuffer | string): Promise<(r: ReadInput) => Result> {
        if (typeof rune == "string") {
            const response = await fetch(rune);
            rune = await response.arrayBuffer();
        }
        const { modelHandlers, log } = this;

        const imports = new ImportsObject(modelHandlers, log);
        const runtime = await Runtime.load(rune, imports);

        return readInputs => {
            imports.setInputs(readInputs);
            runtime.call();

            let outputs = imports.outputs;
            imports.outputs = [];

            return { outputs };
        };
    }
}

export type Result = {
    outputs: Array<OutputValue>,
};

export type OutputValue = {
    id: number,
    value: any,
};

class ImportsObject implements Imports {
    private decoder = new TextDecoder("utf8");
    outputs: Array<any> = [];
    private modelHandlers: Partial<Record<string, ModelConstructor>>;
    private logger: Logger;
    private capabilities: LazyCapability[] = [];

    constructor(
        modelHandlers: Partial<Record<string, ModelConstructor>>,
        logger: Logger,
    ) {
        this.modelHandlers = modelHandlers;
        this.logger = logger;
    }

    setInputs(readInput: ReadInput) {
        const inputs = this.capabilities.map(c => c.description()).map(readInput);

        for (let i = 0; i < this.capabilities.length; i++) {
            this.capabilities[i].value = inputs[i];
        }
    }

    createOutput(type: number): Output {
        const { decoder, outputs } = this;

        // We want the end user to receive all outputs as a return value, but
        // Runes are designed using a callback-based API (it's better for
        // performance). This will create an output which will stash all
        // generated values away in a list so they can be returned at the end.

        return {
            consume(data: Uint8Array) {
                const json = decoder.decode(data);
                outputs.push(json);
            }
        }
    }

    createCapability(type: number): Capability {
        const pair = Object.entries(Capabilities).find(pair => pair[1] == type);
        if (!pair) {
            throw new Error(`Unable to handle capability number ${type}`);
        }

        const capabilityType = pair[0];
        const cap = new LazyCapability(capabilityType as CapabilityType);
        this.capabilities.push(cap);

        return cap;
    }

    createModel(mimetype: string, model: ArrayBuffer): Promise<Model> {
        const handler = this.modelHandlers[mimetype];

        if (!handler) {
            throw new Error(`No handler registered for "${mimetype}" models`);
        }

        return handler(model);
    }

    log(message: string): void {
        this.logger(message);
    }
}

class LazyCapability implements Capability {
    type: CapabilityType;
    value?: Tensor;
    args: Record<string, number> = {};

    constructor(type: CapabilityType) {
        this.type = type;
    }

    description(): InputDescription {
        return {
            type: this.type,
            args: this.args,
        };
    }

    generate(dest: Uint8Array): void {
        if (!this.value) {
            throw new Error();
        }
        dest.set(this.value.dataSync());
    }

    setParameter(name: string, value: number): void {
        this.args[name] = value;
    }
}

import { Capability, Imports, Model, Output, Runtime } from ".";

const Capabilities = {
    "rand": 1,
    "sound": 2,
    "accel": 3,
    "image": 4,
    "raw": 5,
}

type CapabilityConstructor = () => Capability;
type ModelConstructor = (model: ArrayBuffer) => Promise<Model>;
type Logger = (message: string) => void;

export default class Builder {
    private capabilities: Partial<Record<string, CapabilityConstructor>> = {};
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
     * Add support for a capability.
     * @param cap The name of the capability type being provided.
     * @param constructor A function that can construct a new instance of this
     * capability.
     * @returns
     */
    public withCapability<C extends keyof typeof Capabilities>(cap: C, constructor: CapabilityConstructor): this {
        const capabilityType = Capabilities[cap];
        this.capabilities[capabilityType] = constructor;

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

    public async build<T = any>(rune: ArrayBuffer | string, postprocess?: (output: any) => T): Promise<() => Result<T>> {
        if (typeof rune == "string") {
            const response = await fetch(rune);
            rune = await response.arrayBuffer();
        }

        const imports = new ImportsObject(this.capabilities, this.modelHandlers, this.log);

        const runtime = await Runtime.load(rune, imports);

        return () => {
            runtime.call();

            let outputs = imports.outputs;
            imports.outputs = [];

            if (postprocess) {
                outputs = outputs.map(postprocess);
            }

            return { outputs };
        };
    }
}

export type Result<T = any> = {
    outputs: Array<OutputValue<T>>,
};

export type OutputValue<T> = {
    id: number,
    value: T,
};

class ImportsObject implements Imports {
    private decoder = new TextDecoder("utf8");
    outputs: Array<any> = [];
    private capabilities: Partial<Record<string, CapabilityConstructor>>;
    private modelHandlers: Partial<Record<string, ModelConstructor>>;
    private logger: Logger;

    constructor(
        capabilities: Partial<Record<string, CapabilityConstructor>>,
        modelHandlers: Partial<Record<string, ModelConstructor>>,
        logger: Logger,
    ) {
        this.capabilities = capabilities;
        this.modelHandlers = modelHandlers;
        this.logger = logger;
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
        const constructor = this.capabilities[type];

        if (!constructor) {
            // TODO: Convert from capability type to human-friendly name
            throw new Error(`No support was provided for capability type ${type}`);
        }

        return constructor();
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

import { Capability } from "../Runtime";

type Properties = {
    width: number,
    height: number,
};

export class WebcamCapability implements Capability {
    lastImage: any;
    properties: Properties = {
        width: 320,
        height: 320,
    };

    generate(dest: Uint8Array): void {
        // TODO: Figure out how to read from the webcam.
        throw new Error("Method not implemented.");
    }

    setParameter(name: string, value: number): void {
        const properties: Record<string, number> = this.properties;
        properties[name] = value;
    }
}

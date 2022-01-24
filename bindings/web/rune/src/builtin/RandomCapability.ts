import { Capability } from "../Runtime";

export class RandomCapability implements Capability {
    setParameter(name: string, value: number): void {
        // Note: we don't have any configurable settings
    }

    generate(dest: Uint8Array): void {
        window.crypto.getRandomValues(dest);
    }
}

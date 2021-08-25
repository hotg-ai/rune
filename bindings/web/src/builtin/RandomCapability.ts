import { Capability } from "..";

export class RandomCapability implements Capability {
    generate(dest: Uint8Array, id: number): void {
        window.crypto.getRandomValues(dest);
    }
}

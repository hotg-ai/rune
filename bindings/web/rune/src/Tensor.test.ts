import { Shape, Tensor } from ".";

describe("Tensor", () => {
    it("can be round tripped as a Uint8Array", () => {
        const shape = new Shape("u8", [2, 3]);
        const tensor = new Tensor(shape, new Uint8Array([1, 2, 3, 4, 5, 6]));

        const typed = tensor.asTypedArray("u8");

        expect(Array.from(typed)).toEqual([1, 2, 3, 4, 5, 6]);
    });

    it("can be viewed as a Float32Array", () => {
        const raw = new Uint8Array([0, 0, 64, 64]);
        const shape = new Shape("f32", [1]);

        const tensor = new Tensor(shape, raw);
        const typed = tensor.asTypedArray("f32");

        expect(Array.from(typed)).toEqual([3.0]);
    });

    it("can be a slice from a larger buffer", () => {
        const numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        const buffer = new Float32Array(numbers);
        const section = buffer.subarray(3, 6);
        const shape = new Shape("f32", [3]);

        const tensor = new Tensor(shape, new Uint8Array(section.buffer, section.byteOffset, section.byteLength));
        const typed = tensor.asTypedArray("f32");

        expect(Array.from(typed)).toEqual(numbers.slice(3, 6));
    });
});

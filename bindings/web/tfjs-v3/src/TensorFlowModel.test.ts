import { Shape } from "@hotg-ai/rune";
import { assertSameShape } from "./TensorFlowModel";
import * as tfjs from "@tensorflow/tfjs-core";

describe("Tensor Shape assertions", () => {
    it("checks for different dimensions", () => {
        const tensor = tfjs.tensor([1, 2, 3, 4, 5, 6], [2, 3], "float32");

        assertSameShape(tensor, new Shape("f32", [2, 3]));
        expect(() => assertSameShape(tensor, new Shape("f32", [1, 2, 3]))).toThrow();
        expect(() => assertSameShape(tensor, new Shape("f32", [6]))).toThrow();
    });

    it("checks for incompatible types", () => {
        const tensor = tfjs.tensor([1, 2, 3, 4, 5, 6], [2, 3], "float32");

        assertSameShape(tensor, new Shape("f32", [2, 3]));
        expect(() => assertSameShape(tensor, new Shape("u8", [2, 3]))).toThrow();
        expect(() => assertSameShape(tensor, new Shape("utf8", [2, 3]))).toThrow();
    });
});

import Shape from "./Shape";

describe("Shape", () => {
    it("can parse u8[1, 2,3]", () => {
        const text = "u8[1, 2,3]";

        const got = Shape.parse(text);

        expect(got).toEqual(new Shape("u8", [1, 2, 3]));
    });

    const knownShapes = ["u8[1]", "f32[2, 4, 6, 8]"]

    test.each(knownShapes)(`can round-trip %p`, input => {
        const parsed = Shape.parse(input);
        const stringified = parsed.toString();

        expect(stringified).toEqual(input);
    });
});

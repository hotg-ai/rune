import Shape from "./Shape";

describe("Shape", () => {
    it("can parse u8[1, 2,3]", () => {
        const text = "u8[1, 2,3]";

        const got = Shape.parse(text);

        expect(got).toEqual(new Shape("u8", [1, 2, 3]));
    });
});

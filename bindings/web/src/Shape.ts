const ByteSize = {
    "f64": 8,
    "i64": 8,
    "u64": 8,
    "f32": 4,
    "i32": 4,
    "u32": 4,
    "u16": 2,
    "i16": 2,
    "u8": 1,
    "i8": 1
} as const;

/**
 * A description of a tensor.
 */
export default class Shape {
    /**
     * The element type.
     */
    readonly type: string;
    /**
     * The tensor's dimensions.
     */
    readonly dimensions: number[];

    constructor(type: string, values: number[]) {
        this.type = type;
        this.dimensions = values;
    }

    /**
     * Parse a string like "u8[1, 2, 3]" into a Shape.
     */
    static parse(text: string): Shape {
        const pattern = /^([\w\d]+)\[(\d+(?:,\s*\d+)*)\]$/;
        const match = pattern.exec(text.replace(" ", ""));

        if (!match) {
            throw new Error();
        }

        const [_, typeName, dims] = match;

        checkElementType(typeName, text);

        return new Shape(typeName, dims.split(",").map(d => parseInt(d.trim())));
    }

    /**
     * The number of dimensions this tensor has.
     */
    get rank(): number {
        return this.dimensions.length;
    }

    /**
     * The number of elements in this tensor.
     */
    get tensorSize(): number {
        return this.dimensions.reduce((product, dim) => product * dim, 1);
    }

    /**
     * The number of bytes used to store this tensor's elements.
     */
    get byteSize(): number {
        const sizes: Record<string, number | undefined> = ByteSize;
        const elementSize = sizes[this.type] || 1;
        return this.tensorSize * elementSize;
    }

    toString(): string {
        const { type, dimensions } = this;
        const dims = dimensions.join(", ");
        return `${type}[${dims}]`;
    }
}


function checkElementType(typeName: string, input: string) {
    const knownElements = Object.keys(ByteSize);

    if (typeName in ByteSize) {
        return;
    }

    console.warn(`The "${typeName}" in "${input}" isn't one of the known element types (${knownElements})`);
}

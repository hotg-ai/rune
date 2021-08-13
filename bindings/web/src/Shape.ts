const ByteSize: Record<string, number> = {
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
};

export default class Shape {
    type: string;
    values: number[];

    constructor(type: string, values: number[]) {
        this.type = type;
        this.values = values;
    }

    static parse(dimensions: string): Shape {
        const pattern = /^([\w\d]+)\[(\d+(?:,\s*\d+)*)\]$/;
        const match = pattern.exec(dimensions.replace(" ", ""));

        if (!match) {
            throw new Error();
        }

        const [_, typeName, dims] = match;

        return new Shape(typeName!, dims.split(",").map(d => parseInt(d.trim())));
    }

    get tensorSize(): number {
        return this.values.reduce((product, dim) => product * dim, 1);
    }

    get byteSize(): number {
        return this.tensorSize * ByteSize[this.type];
    }
}


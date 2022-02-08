import Shape from "./Shape";

// Some versions of Safari doesn't support BigUint64Array and friends, and
// it's not possible to polyfill these types because bigint is a builtin type.
//
// This workaround lets us use them when possible and throws an exception at
// runtime when they aren't.
const BigUint64ArrayShim = global.BigUint64Array ?? class { constructor() { throw new Error("BigUint64Array is not supported on this device"); } };
const BigInt64ArrayShim = global.BigInt64Array ?? class { constructor() { throw new Error("BigInt64Array is not supported on this device"); } };

const typedArrays = {
    "f64": Float64Array,
    "i64": BigInt64ArrayShim,
    "u64": BigUint64ArrayShim,
    "f32": Float32Array,
    "i32": Int32Array,
    "u32": Uint32Array,
    "u16": Uint16Array,
    "i16": Int16Array,
    "u8": Uint8ClampedArray,
    "i8": Int8Array,
} as const;

export default class Tensor {
    public readonly elements: Uint8Array;
    public readonly shape: Shape;

    constructor(shape: Shape, elements: Uint8Array) {
        this.shape = shape;
        this.elements = elements;
    }

    public asTypedArray(elementType: "f64"): Float64Array;
    public asTypedArray(elementType: "i64"): BigInt64Array;
    public asTypedArray(elementType: "u64"): BigUint64Array;
    public asTypedArray(elementType: "f32"): Float32Array;
    public asTypedArray(elementType: "i32"): Int32Array;
    public asTypedArray(elementType: "u32"): Uint32Array;
    public asTypedArray(elementType: "u16"): Uint16Array;
    public asTypedArray(elementType: "i16"): Int16Array;
    public asTypedArray(elementType: "u8"): Uint8ClampedArray;
    public asTypedArray(elementType: "i8"): Int8Array;

    public asTypedArray(elementType: keyof typeof typedArrays): ArrayBuffer {
        if (this.shape.type != elementType) {
            throw new Error(`Attempting to interpret a ${this.shape.toString()} as a ${elementType} tensor`);
        }

        const { buffer, byteOffset, byteLength } = this.elements;
        const length = byteLength / Shape.ByteSize[this.shape.type];
        const constructor = typedArrays[elementType];

        return new constructor(buffer, byteOffset, length);
    }

    public get elementType(): string {
        return this.shape.type;
    }

    public get dimensions(): readonly number[] {
        return this.shape.dimensions;
    }
}

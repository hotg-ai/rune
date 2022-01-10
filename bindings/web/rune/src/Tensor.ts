import Shape from "./Shape";

const typedArrays = {
    "f64": Float64Array,
    "i64": BigInt64Array,
    "u64": BigUint64Array,
    "f32": Float32Array,
    "i32": Int32Array,
    "u32": Uint32Array,
    "u16": Uint16Array,
    "i16": Int16Array,
    "u8": Uint8ClampedArray,
    "i8": Int8Array,
} as const;

export default class Tensor {
    public readonly elements: ArrayBuffer;
    public readonly shape: Shape;

    constructor(shape: Shape, elements: ArrayBuffer) {
        this.shape = shape;
        // Note: We *need* elements to be an ArrayBuffer so the typed array
        // conversion creates a view instead of copying the buffer (i.e. if we
        // pass a Uint8Array to newFloat32Array() it will see a list of numbers
        // and use each as a 32-bit float).
        this.elements = ArrayBuffer.isView(elements) ? elements.buffer : elements;
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

        const constructor = typedArrays[elementType];
        return new constructor(this.elements);
    }

    public get elementType(): string {
        return this.shape.type;
    }

    public get dimensions(): readonly number[] {
        return this.shape.dimensions;
    }
}

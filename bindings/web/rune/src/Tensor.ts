import Shape from "./Shape";

// Some versions of Safari doesn't support BigUint64Array and friends, and
// it's not possible to polyfill these types because bigint is a builtin type.
//
// This workaround lets us use them when possible and throws an exception at
// runtime when they aren't.
const BigUint64ArrayShim = global.BigUint64Array ?? class { constructor() { throw new Error("BigUint64Array is not supported on this device"); } };
const BigInt64ArrayShim = global.BigInt64Array ?? class { constructor() { throw new Error("BigInt64Array is not supported on this device"); } };

const typedArrayConstructors = {
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

type TypedArrayConstructors = typeof typedArrayConstructors;

export type TypedArrays = {
    [Key in keyof TypedArrayConstructors]: InstanceType<TypedArrayConstructors[Key]>;
}

/**
 * An opaque tensor.
 */
export default class Tensor {
    /**
     * The raw bytes containing the tensor data.
     */
    public readonly elements: Uint8Array;
    /**
     * The tensor's shape (element type and dimensions).
     */
    public readonly shape: Shape;

    constructor(shape: Shape, elements: Uint8Array) {
        this.shape = shape;
        this.elements = elements;
    }

    /**
     * Construct a new Tensor from a typed array containing its flattened
     * elements in row-major order.
     *
     * @param elementType The type of the element
     * @param dimensions The tensor's dimensions
     * @param elements The elements
     * @returns
     */
    public static fromTypedArray<S extends keyof TypedArrays>(
        elementType: S,
        dimensions: readonly number[],
        elements: TypedArrays[S],
    ): Tensor {
        const { buffer, byteLength, byteOffset } = elements;
        const shape = new Shape(elementType, [...dimensions]);
        return new Tensor(shape, new Uint8Array(buffer, byteOffset, byteLength));
    }

    /**
     * View this tensor's data as an array of 64-bit floats.
     *
     * This will fail if this isn't a f64 tensor.
     */
    public asTypedArray(elementType: "f64"): Float64Array;
    /**
     * View this tensor's data as an array of 64-bit signed integers.
     *
     * This will fail if this isn't a i64 tensor. It may also fail on
     * versions of Safari because they don't support BigInt64Array.
     */
    public asTypedArray(elementType: "i64"): BigInt64Array;
    /**
     * View this tensor's data as an array of 64-bit unsigned integers.
     *
     * This will fail if this isn't a u64 tensor. It may also fail on
     * versions of Safari because they don't support BigUint64Array.
     */
    public asTypedArray(elementType: "u64"): BigUint64Array;
    /**
     * View this tensor's data as an array of 32-bit floats.
     *
     * This will fail if this isn't a f32 tensor.
     */
    public asTypedArray(elementType: "f32"): Float32Array;
    /**
     * View this tensor's data as an array of 32-bit signed integers.
     *
     * This will fail if this isn't a i32 tensor.
     */
    public asTypedArray(elementType: "i32"): Int32Array;
    /**
     * View this tensor's data as an array of 32-bit unsigned integers.
     *
     * This will fail if this isn't a u32 tensor.
     */
    public asTypedArray(elementType: "u32"): Uint32Array;
    /**
     * View this tensor's data as an array of 16-bit signed integers.
     *
     * This will fail if this isn't a i16 tensor.
     */
    public asTypedArray(elementType: "i16"): Int16Array;
    /**
     * View this tensor's data as an array of 16-bit unsigned integers.
     *
     * This will fail if this isn't a u16 tensor.
     */
    public asTypedArray(elementType: "u16"): Uint16Array;
    /**
     * View this tensor's data as an array of 8-bit signed integers.
     *
     * This will fail if this isn't a i8 tensor.
     */
    public asTypedArray(elementType: "i8"): Int8Array;
    /**
     * View this tensor's data as an array of 8-bit unsigned integers.
     *
     * This will fail if this isn't a u8 tensor.
     */
    public asTypedArray(elementType: "u8"): Uint8ClampedArray;

    public asTypedArray(elementType: keyof typeof typedArrayConstructors): ArrayBuffer {
        if (this.shape.type != elementType) {
            throw new Error(`Attempting to interpret a ${this.shape.toString()} as a ${elementType} tensor`);
        }

        const { buffer, byteOffset, byteLength } = this.elements;
        const length = byteLength / Shape.ByteSize[this.shape.type];
        const constructor = typedArrayConstructors[elementType];

        return new constructor(buffer, byteOffset, length);
    }

    public get elementType(): string {
        return this.shape.type;
    }

    public get dimensions(): readonly number[] {
        return this.shape.dimensions;
    }
}

const x = Tensor.fromTypedArray

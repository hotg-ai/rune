export function toTypedArray(typeName: "f64", data: Uint8Array): Float64Array;
export function toTypedArray(typeName: "f32", data: Uint8Array): Float32Array;

export function toTypedArray(typeName: "i8", data: Uint8Array): Int8Array;
export function toTypedArray(typeName: "i16", data: Uint8Array): Int16Array;
export function toTypedArray(typeName: "i32", data: Uint8Array): Int32Array;
export function toTypedArray(typeName: "i64", data: Uint8Array): BigInt64Array;

export function toTypedArray(typeName: "u8", data: Uint8Array): Uint8Array;
export function toTypedArray(typeName: "u16", data: Uint8Array): Uint16Array;
export function toTypedArray(typeName: "u32", data: Uint8Array): Uint32Array;
export function toTypedArray(typeName: "u64", data: Uint8Array): BigUint64Array;

export function toTypedArray(typeName: string, data: Uint8Array): any;

export function toTypedArray(typeName: string, data: Uint8Array): any {
    let { buffer, byteOffset, byteLength } = data;
    const bytes = buffer.slice(byteOffset, byteOffset + byteLength);

    switch (typeName) {
        case "f32":
            return new Float32Array(bytes);
        case "f64":
            return new Float64Array(bytes);

        case "i8":
            return new Int8Array(bytes);
        case "i16":
            return new Int16Array(bytes);
        case "i32":
            return new Int32Array(bytes);
        case "i64":
            return new BigInt64Array(bytes);

        case "u8":
            return new Uint8Array(bytes);
        case "u16":
            return new Uint16Array(bytes);
        case "u32":
            return new Uint32Array(bytes);
        case "u64":
            return new BigUint64Array(bytes);

        default:
            throw new Error(`Unknown tensor type: ${typeName}`);
    }
}

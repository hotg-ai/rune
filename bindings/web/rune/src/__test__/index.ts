import pino, { Logger } from "pino";
import { ElementType, Tensor } from "..";
import { isTensor } from "../utils";

function stringArray(buffer: ArrayBuffer, byteOffset: number, length: number) {
  const reader = new DataView(buffer, byteOffset, length);
  const decoder = new TextDecoder();
  const strings: string[] = [];

  let offset = 0;
  while (offset < reader.byteLength) {
    const length = reader.getUint32(offset, true);
    const utf8 = new Uint8Array(buffer, byteOffset + offset, length);
    strings.push(decoder.decode(utf8));
    offset += 4 + length;
  }

  return strings;
}

function typedArray<T>(constructor: {
  new (
    buffer: ArrayBufferLike,
    byteOffset: number,
    length: number
  ): ArrayLike<T>;
  readonly BYTES_PER_ELEMENT: number;
}): (b: ArrayBuffer, off: number, len: number) => T[] {
  return (b, off, len) =>
    Array.from(new constructor(b, off, len / constructor.BYTES_PER_ELEMENT));
}

type NumericTensor = {
  elementType: "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "f32" | "f64";
  dimensions: number[];
  elements: number[];
};

type StringTensor = {
  elementType: "utf8";
  dimensions: number[];
  elements: string[];
};

type BigIntTensor = {
  elementType: "u64" | "i64";
  dimensions: number[];
  elements: bigint[];
};

type FormattedTensor = NumericTensor | StringTensor | BigIntTensor;

export function formatTensor(tensor: Tensor): FormattedTensor {
  const dimensions = Array.from(tensor.dimensions);

  const { buffer, byteOffset, byteLength } = tensor.buffer;

  switch (tensor.elementType) {
    case ElementType.U8:
      return {
        elementType: "u8",
        dimensions,
        elements: typedArray(Uint8Array)(buffer, byteOffset, byteLength),
      };
    case ElementType.I8:
      return {
        elementType: "i8",
        dimensions,
        elements: typedArray(Int8Array)(buffer, byteOffset, byteLength),
      };
    case ElementType.U16:
      return {
        elementType: "u16",
        dimensions,
        elements: typedArray(Uint16Array)(buffer, byteOffset, byteLength),
      };
    case ElementType.I16:
      return {
        elementType: "i16",
        dimensions,
        elements: typedArray(Int16Array)(buffer, byteOffset, byteLength),
      };
    case ElementType.U32:
      return {
        elementType: "u32",
        dimensions,
        elements: typedArray(Uint32Array)(buffer, byteOffset, byteLength),
      };
    case ElementType.I32:
      return {
        elementType: "i32",
        dimensions,
        elements: typedArray(Int32Array)(buffer, byteOffset, byteLength),
      };
    case ElementType.F32:
      return {
        elementType: "f32",
        dimensions,
        elements: typedArray(Float32Array)(buffer, byteOffset, byteLength),
      };
    case ElementType.F64:
      return {
        elementType: "f64",
        dimensions,
        elements: typedArray(Float64Array)(buffer, byteOffset, byteLength),
      };
    case ElementType.U64:
      return {
        elementType: "u64",
        dimensions,
        elements: typedArray(BigUint64Array)(buffer, byteOffset, byteLength),
      };
    case ElementType.I64:
      return {
        elementType: "i64",
        dimensions,
        elements: typedArray(BigInt64Array)(buffer, byteOffset, byteLength),
      };
    case ElementType.Utf8:
      return {
        elementType: "utf8",
        dimensions,
        elements: stringArray(buffer, byteOffset, byteLength),
      };
  }
}

export function testLogger(): Logger {
  const bindings = () => {
    const { currentTestName } = expect.getState();
    return { test: currentTestName };
  };

  const humanReadableTensors = (object: any): any => {
    if (isTensor(object)) {
      return formatTensor(object);
    }

    if (typeof object == "object") {
      const formatted: any = {};

      for (const key in object) {
        formatted[key] = humanReadableTensors(object[key]);
      }

      return formatted;
    }

    return object;
  };

  const logger = pino({
    level: "trace",
    nestedKey: "payload",
    timestamp: false,
    formatters: {
      bindings,
      log: humanReadableTensors,
      level: (label) => ({ level: label }),
    },
  });

  beforeEach(() => {
    const { currentTestName } = expect.getState();
    logger.info({ test: currentTestName }, "Starting Test");
  });

  afterEach(() => {
    const { currentTestName } = expect.getState();
    logger.info({ test: currentTestName }, "Completed Test");
    logger.flush();
  });

  return logger;
}

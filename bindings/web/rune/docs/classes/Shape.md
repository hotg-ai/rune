[@hotg-ai/rune](../README.md) / [Exports](../modules.md) / Shape

# Class: Shape

A description of a tensor.

## Table of contents

### Constructors

- [constructor](Shape.md#constructor)

### Properties

- [dimensions](Shape.md#dimensions)
- [type](Shape.md#type)
- [ByteSize](Shape.md#bytesize)

### Accessors

- [byteSize](Shape.md#bytesize)
- [rank](Shape.md#rank)
- [tensorSize](Shape.md#tensorsize)

### Methods

- [toString](Shape.md#tostring)
- [parse](Shape.md#parse)

## Constructors

### constructor

• **new Shape**(`type`, `values`)

#### Parameters

| Name | Type |
| :------ | :------ |
| `type` | `string` |
| `values` | `number`[] |

## Properties

### dimensions

• `Readonly` **dimensions**: readonly `number`[]

The tensor's dimensions.

___

### type

• `Readonly` **type**: `string`

The element type.

___

### ByteSize

▪ `Static` **ByteSize**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `f32` | ``4`` |
| `f64` | ``8`` |
| `i16` | ``2`` |
| `i32` | ``4`` |
| `i64` | ``8`` |
| `i8` | ``1`` |
| `u16` | ``2`` |
| `u32` | ``4`` |
| `u64` | ``8`` |
| `u8` | ``1`` |

## Accessors

### byteSize

• `get` **byteSize**(): `number`

The number of bytes used to store this tensor's elements.

#### Returns

`number`

___

### rank

• `get` **rank**(): `number`

The number of dimensions this tensor has.

#### Returns

`number`

___

### tensorSize

• `get` **tensorSize**(): `number`

The number of elements in this tensor.

#### Returns

`number`

## Methods

### toString

▸ **toString**(): `string`

#### Returns

`string`

___

### parse

▸ `Static` **parse**(`text`): [`Shape`](Shape.md)

Parse a string like "u8[1, 2, 3]" into a Shape.

#### Parameters

| Name | Type |
| :------ | :------ |
| `text` | `string` |

#### Returns

[`Shape`](Shape.md)

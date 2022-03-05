[@hotg-ai/rune](../README.md) / [Exports](../modules.md) / Tensor

# Class: Tensor

An opaque tensor.

## Table of contents

### Constructors

- [constructor](Tensor.md#constructor)

### Properties

- [elements](Tensor.md#elements)
- [shape](Tensor.md#shape)

### Accessors

- [dimensions](Tensor.md#dimensions)
- [elementType](Tensor.md#elementtype)

### Methods

- [asTypedArray](Tensor.md#astypedarray)

## Constructors

### constructor

• **new Tensor**(`shape`, `elements`)

#### Parameters

| Name | Type |
| :------ | :------ |
| `shape` | [`Shape`](Shape.md) |
| `elements` | `Uint8Array` |

## Properties

### elements

• `Readonly` **elements**: `Uint8Array`

The raw bytes containing the tensor data.

___

### shape

• `Readonly` **shape**: [`Shape`](Shape.md)

The tensor's shape (element type and dimensions).

## Accessors

### dimensions

• `get` **dimensions**(): readonly `number`[]

#### Returns

readonly `number`[]

___

### elementType

• `get` **elementType**(): `string`

#### Returns

`string`

## Methods

### asTypedArray

▸ **asTypedArray**(`elementType`): `Float64Array`

View this tensor's data as an array of 64-bit floats.

This will fail if this isn't a f64 tensor.

#### Parameters

| Name | Type |
| :------ | :------ |
| `elementType` | ``"f64"`` |

#### Returns

`Float64Array`

▸ **asTypedArray**(`elementType`): `BigInt64Array`

View this tensor's data as an array of 64-bit signed integers.

This will fail if this isn't a i64 tensor. It may also fail on
versions of Safari because they don't support BigInt64Array.

#### Parameters

| Name | Type |
| :------ | :------ |
| `elementType` | ``"i64"`` |

#### Returns

`BigInt64Array`

▸ **asTypedArray**(`elementType`): `BigUint64Array`

View this tensor's data as an array of 64-bit unsigned integers.

This will fail if this isn't a u64 tensor. It may also fail on
versions of Safari because they don't support BigUint64Array.

#### Parameters

| Name | Type |
| :------ | :------ |
| `elementType` | ``"u64"`` |

#### Returns

`BigUint64Array`

▸ **asTypedArray**(`elementType`): `Float32Array`

View this tensor's data as an array of 32-bit floats.

This will fail if this isn't a f32 tensor.

#### Parameters

| Name | Type |
| :------ | :------ |
| `elementType` | ``"f32"`` |

#### Returns

`Float32Array`

▸ **asTypedArray**(`elementType`): `Int32Array`

View this tensor's data as an array of 32-bit signed integers.

This will fail if this isn't a i32 tensor.

#### Parameters

| Name | Type |
| :------ | :------ |
| `elementType` | ``"i32"`` |

#### Returns

`Int32Array`

▸ **asTypedArray**(`elementType`): `Uint32Array`

View this tensor's data as an array of 32-bit unsigned integers.

This will fail if this isn't a u32 tensor.

#### Parameters

| Name | Type |
| :------ | :------ |
| `elementType` | ``"u32"`` |

#### Returns

`Uint32Array`

▸ **asTypedArray**(`elementType`): `Int16Array`

View this tensor's data as an array of 16-bit signed integers.

This will fail if this isn't a i16 tensor.

#### Parameters

| Name | Type |
| :------ | :------ |
| `elementType` | ``"i16"`` |

#### Returns

`Int16Array`

▸ **asTypedArray**(`elementType`): `Uint16Array`

View this tensor's data as an array of 16-bit unsigned integers.

This will fail if this isn't a u16 tensor.

#### Parameters

| Name | Type |
| :------ | :------ |
| `elementType` | ``"u16"`` |

#### Returns

`Uint16Array`

▸ **asTypedArray**(`elementType`): `Int8Array`

View this tensor's data as an array of 8-bit signed integers.

This will fail if this isn't a i8 tensor.

#### Parameters

| Name | Type |
| :------ | :------ |
| `elementType` | ``"i8"`` |

#### Returns

`Int8Array`

▸ **asTypedArray**(`elementType`): `Uint8ClampedArray`

View this tensor's data as an array of 8-bit unsigned integers.

This will fail if this isn't a u8 tensor.

#### Parameters

| Name | Type |
| :------ | :------ |
| `elementType` | ``"u8"`` |

#### Returns

`Uint8ClampedArray`

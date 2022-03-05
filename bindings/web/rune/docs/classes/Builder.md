[@hotg-ai/rune](../README.md) / [Exports](../modules.md) / Builder

# Class: Builder

A builder object which can be used to initialize the Rune runtime.

## Table of contents

### Constructors

- [constructor](Builder.md#constructor)

### Properties

- [log](Builder.md#log)
- [modelHandlers](Builder.md#modelhandlers)

### Methods

- [build](Builder.md#build)
- [onDebug](Builder.md#ondebug)
- [withModelHandler](Builder.md#withmodelhandler)

## Constructors

### constructor

• **new Builder**()

## Properties

### log

• `Private` **log**: `Logger`

___

### modelHandlers

• `Private` **modelHandlers**: `Partial`<`Record`<`string`, `ModelConstructor`\>\> = `{}`

## Methods

### build

▸ **build**(`rune`): `Promise`<[`Evaluate`](../modules.md#evaluate)\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `rune` | `string` \| `ArrayBuffer` |

#### Returns

`Promise`<[`Evaluate`](../modules.md#evaluate)\>

___

### onDebug

▸ **onDebug**(`handler`): [`Builder`](Builder.md)

Set a handler that will be called every time the Rune logs a message.

#### Parameters

| Name | Type |
| :------ | :------ |
| `handler` | `Logger` |

#### Returns

[`Builder`](Builder.md)

___

### withModelHandler

▸ **withModelHandler**(`mimetype`, `constructor`): [`Builder`](Builder.md)

Add support for a new type of model.

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `mimetype` | `string` | The "mimetype" that specifies which type of model being handled. |
| `constructor` | `ModelConstructor` | A constructor which will load the model. |

#### Returns

[`Builder`](Builder.md)

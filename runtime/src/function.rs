use anyhow::{Context, Error};
use crate::{WasmType, WasmValue};

#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    parameters: &'static [WasmType],
    returns: &'static [WasmType],
}

impl Signature {
    pub fn parameters(&self) -> &'static [WasmType] { self.parameters }

    pub fn returns(&self) -> &'static [WasmType] { self.returns }
}

/// A type-erased function which can be used with WebAssembly values.
pub struct Function {
    signature: Signature,
    func: Box<
        dyn Fn(&[WasmValue]) -> Result<Vec<WasmValue>, Error>
            + Send
            + Sync
            + 'static,
    >,
}

impl Function {
    pub fn new<F, Args, Rets>(closure: F) -> Self
    where
        F: Fn(Args) -> Result<Rets, Error> + Sync + Send + 'static,
        Args: WasmTypeList,
        Rets: WasmTypeList,
    {
        let signature = Signature {
            parameters: Args::TYPES,
            returns: Rets::TYPES,
        };
        let func = Box::new(
            move |args: &[WasmValue]| -> Result<Vec<WasmValue>, Error> {
                let args = Args::from_values(args)
                    .context("Unable to unpack the arguments")?;

                let returns = closure(args)?;
                Ok(returns.into_values())
            },
        );

        Function { signature, func }
    }

    pub fn signature(&self) -> &Signature { &self.signature }

    pub fn call(&self, args: &[WasmValue]) -> Result<Vec<WasmValue>, Error> {
        (self.func)(args)
    }
}

/// A list of WebAssembly types, typically used for function arguments or return
/// values.
pub trait WasmTypeList: Sized {
    const TYPES: &'static [WasmType];

    fn into_values(self) -> Vec<WasmValue>;
    fn from_values(values: &[WasmValue]) -> Result<Self, FromValuesError>;
}

/// A type that can be converted to and from WebAssembly primitives.
pub trait ToFromWasmType: Sized {
    const WASM_TYPE: WasmType;

    fn to_value(self) -> WasmValue;
    fn from_value(v: WasmValue) -> Option<Self>;
}

#[derive(Debug, Copy, Clone, PartialEq, thiserror::Error)]
pub enum FromValuesError {
    #[error(
        "Incorrect number of elements, expected {} but found {}",
        expected,
        actual
    )]
    IncorrectArity { expected: usize, actual: usize },
    #[error(
        "Value {} should have been a {:?} but was actually {}",
        index,
        expected,
        actual
    )]
    IncorrectType {
        index: usize,
        expected: WasmType,
        actual: WasmValue,
    },
}

macro_rules! impl_wasm_type_list {
    ($($letters:ident),* $(,)?) => {
        impl<$($letters),*> WasmTypeList for ($($letters,)*)
        where
            $(
                $letters : ToFromWasmType
            ),*
        {
            const TYPES: &'static [WasmType] = &[
                $(
                    $letters::WASM_TYPE
                ),*
            ];

            #[allow(non_snake_case)]
            fn into_values(self) -> Vec<WasmValue> {
                let ($($letters,)*) = self;

                vec![
                    $($letters.to_value()),*
                ]
            }

            #[allow(unused_assignments, unused_variables, unused_mut, non_snake_case)]
            fn from_values(values: &[WasmValue]) -> Result<Self, FromValuesError> {
                match values {
                    [$($letters),*] => {
                        let mut letter_number = 0;
                        $(
                            let $letters = <$letters>::from_value(*$letters)
                                .ok_or(FromValuesError::IncorrectType {
                                    index: letter_number,
                                    expected: <$letters>::WASM_TYPE,
                                    actual: *$letters,
                                })?;
                            letter_number += 1;
                        )*

                        Ok(($($letters,)*))
                    },
                    _ => todo!(),
                }
            }
        }
    };
}

macro_rules! impl_to_from_wasm_type {
    ($( $variant:ident => $type:ty),* $(,)?) => {
        $(
            impl ToFromWasmType for $type {
                const WASM_TYPE: WasmType = WasmType::$variant;

                fn to_value(self) -> WasmValue {
                    WasmValue::$variant(self)
                }
                fn from_value(v: WasmValue) -> Option<Self> {
                    match v {
                        WasmValue::$variant(value) => Some(value),
                        _ => None,
                    }
                }
            }

            impl WasmTypeList for $type {
                const TYPES: &'static [WasmType] = &[WasmType::$variant];

                fn into_values(self) -> Vec<WasmValue> { vec![self.to_value()] }

                fn from_values(values: &[WasmValue]) -> Result<Self, FromValuesError> {
                    match values {
                        [WasmValue::$variant(value)] => Ok(*value),
                        [other] => Err(FromValuesError::IncorrectType {
                            index: 0,
                            expected: WasmType::$variant,
                            actual: *other,
                        }),
                        other => Err(FromValuesError::IncorrectArity {
                            expected: 1,
                            actual: other.len(),
                        })
                    }
                }
            }
        )*
    };
}

impl_wasm_type_list!();
impl_wasm_type_list!(A);
impl_wasm_type_list!(A, B);
impl_wasm_type_list!(A, B, C);
impl_wasm_type_list!(A, B, C, D);
impl_wasm_type_list!(A, B, C, D, E);
impl_wasm_type_list!(A, B, C, D, E, F);
impl_wasm_type_list!(A, B, C, D, E, F, G);
impl_wasm_type_list!(A, B, C, D, E, F, G, H);
impl_wasm_type_list!(A, B, C, D, E, F, G, H, I);
impl_wasm_type_list!(A, B, C, D, E, F, G, H, I, J);
impl_wasm_type_list!(A, B, C, D, E, F, G, H, I, J, K);

impl_to_from_wasm_type!(I32 => i32, I64 => i64, F32 => f32, F64 => f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_tuples_to_wasm_values() {
        let src = (1_i32, 2_f32, 3_i64, 4_f64);
        let should_be = vec![
            WasmValue::I32(1),
            WasmValue::F32(2.0),
            WasmValue::I64(3),
            WasmValue::F64(4.0),
        ];

        let got = src.into_values();

        assert_eq!(got, should_be);
    }
}

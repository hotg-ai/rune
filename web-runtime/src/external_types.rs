use std::fmt::{self, Formatter};

use anyhow::Error;
use fmt::Debug;
use wasm_bindgen::prelude::*;
use js_sys::{Function, Uint8Array};

use crate::rust_error;

#[wasm_bindgen(typescript_custom_section)]
const CAPABILITY: &'static str = r#"
export type CapabilityConstructor = () => Capability;

export interface Capability {
    generate(buffer: Uint8Array): number;
    setProperty(key: string, value: any): void;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Function, typescript_type = CapabilityConstructor)]
    pub type CapabilityConstructor;

    #[wasm_bindgen(typescript_type = Capability)]
    pub type Capability;

    #[wasm_bindgen(method, catch)]
    pub fn generate(
        this: &Capability,
        buffer: Uint8Array,
    ) -> Result<u32, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "setProperty")]
    pub fn set_property(
        this: &Capability,
        key: &str,
        value: JsValue,
    ) -> Result<(), JsValue>;
}

impl Debug for Capability {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let js: &JsValue = self.as_ref();
        Debug::fmt(js, f)
    }
}

impl rune_runtime::Capability for Capability {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        // Safety: We only use the Uint8Array mutably for the duration of this
        // call and JS has no way of accessing the original &mut [u8].
        unsafe {
            let buffer =
                Uint8Array::view_mut_raw(buffer.as_mut_ptr(), buffer.len());
            let bytes_written =
                <Capability>::generate(self, buffer).map_err(rust_error)?;
            Ok(bytes_written as usize)
        }
    }

    fn set_parameter(
        &mut self,
        _name: &str,
        _value: runic_types::Value,
    ) -> Result<(), rune_runtime::ParameterError> {
        todo!()
    }
}

#[wasm_bindgen(typescript_custom_section)]
const OUTPUT: &'static str = r#"
export type OutputConstructor = () => Output;

export interface Output {
    consume(data: Uint8Array): void;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Function, typescript_type = OutputConstructor)]
    pub type OutputConstructor;

    #[wasm_bindgen(typescript_type = Output)]
    pub type Output;

    #[wasm_bindgen(method, catch)]
    pub fn consume(this: &Output, data: Uint8Array) -> Result<(), JsValue>;
}

impl Debug for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let js: &JsValue = self.as_ref();
        Debug::fmt(js, f)
    }
}

impl rune_runtime::Output for Output {
    fn consume(&mut self, buffer: &[u8]) -> Result<(), Error> {
        let buffer = Uint8Array::from(buffer);
        <Output>::consume(self, buffer).map_err(rust_error)
    }
}

#[wasm_bindgen(typescript_custom_section)]
const MODEL: &'static str = r#"
export type ModelConstructor = () => Model;

export interface Model {
    infer(input: Uint8Array, output: Uint8Array): void;
 }
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Function, typescript_type = ModelConstructor)]
    pub type ModelConstructor;

    #[wasm_bindgen(typescript_type = Model)]
    pub type Model;

    #[wasm_bindgen(method, catch)]
    pub fn infer(
        this: &Output,
        input: Uint8Array,
        output: Uint8Array,
    ) -> Result<(), JsValue>;
}

// Safety: This works as long as JavaScript is single-threaded

unsafe impl Send for Capability {}
unsafe impl Sync for Capability {}
unsafe impl Send for Output {}
unsafe impl Sync for Output {}
unsafe impl Send for Model {}
unsafe impl Sync for Model {}

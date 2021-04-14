use wasm_bindgen::prelude::*;
use js_sys::{Function, Uint8Array};

#[wasm_bindgen(typescript_custom_section)]
const CAPABILITY: &'static str = r#"
type CapabilityConstructor = () => Capability;

interface Capability {
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

#[wasm_bindgen(typescript_custom_section)]
const OUTPUT: &'static str = r#"
type OutputConstructor = () => Output;

interface Output {
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

#[wasm_bindgen(typescript_custom_section)]
const MODEL: &'static str = r#"
type ModelConstructor = () => Promise<Model>;

interface Model { }
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Function, typescript_type = ModelConstructor)]
    pub type ModelConstructor;

    #[wasm_bindgen(typescript_type = Model)]
    pub type Model;
}

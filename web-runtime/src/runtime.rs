use std::{collections::HashMap, sync::Mutex};
use rune_runtime::Image;
use wasm_bindgen::{JsCast, prelude::*};
use js_sys::{
    JsString, Object, Reflect,
    WebAssembly::{self, Instance, Memory},
};
use alloc::sync::Arc;
use crate::Imports;

#[wasm_bindgen]
pub struct Runtime {
    instance: Instance,
    drops: Vec<Box<dyn Drop>>,
}

#[wasm_bindgen]
impl Runtime {
    pub async fn load(
        rune: Vec<u8>,
        imports: Imports,
    ) -> Result<Runtime, JsValue> {
        let memory: Arc<Mutex<Option<Memory>>> = Arc::new(Mutex::new(None));
        let image = imports.to_image();

        let mut registrar = Registrar::new(Arc::clone(&memory));
        image.initialize_imports(&mut registrar);

        let (imports, drops) = registrar.to_object()?;
        let pending = WebAssembly::instantiate_buffer(&rune, &imports);
        let instance = wasm_bindgen_futures::JsFuture::from(pending).await?;

        Ok(Runtime {
            instance: instance.dyn_into()?,
            drops,
        })
    }

    pub fn call(&mut self) -> Result<(), JsValue> {
        let exports = self.instance.exports();

        let name = JsValue::from_str("call");
        let call_func =
            Reflect::get(&exports, &name)?.dyn_into::<js_sys::Function>()?;

        let zero = JsValue::from_f64(0.0);
        call_func.call3(&JsValue::NULL, &zero, &zero, &zero)?;

        Ok(())
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        for thing in self.drops.drain(..) {
            drop(thing);
        }
    }
}

#[derive(Default)]
struct Registrar {
    memory: Arc<Mutex<Option<Memory>>>,
    namespaces: HashMap<String, Object>,
    drops: Vec<Box<dyn Drop>>,
}

impl Registrar {
    fn new(memory: Arc<Mutex<Option<Memory>>>) -> Self {
        Registrar {
            memory,
            namespaces: HashMap::new(),
            drops: Vec::new(),
        }
    }

    fn to_object(self) -> Result<(Object, Vec<Box<dyn Drop>>), JsValue> {
        let Registrar {
            memory: _,
            namespaces,
            drops,
        } = self;
        let obj = Object::new();

        for (key, value) in namespaces {
            let key = JsString::from(key);
            Reflect::set(&obj, &key, &value)?;
        }

        Ok((obj, drops))
    }
}

impl rune_runtime::Registrar for Registrar {
    fn register_function(
        &mut self,
        namespace: &str,
        name: &str,
        function: rune_runtime::Function,
    ) {
        let memory = Arc::clone(&self.memory);
        let ns = self
            .namespaces
            .entry(namespace.to_string())
            .or_insert_with(Object::new);
        let name = JsValue::from_str(name);

        let droppable = crate::hacks::with_wrapped_closure(function, memory, |value| {
            if let Err(e) = Reflect::set(ns, &name, value) {
                wasm_bindgen::throw_val(e);
            }
        });

        // we store the closures so we can clean them up properly later on
        self.drops.push(droppable);
    }
}

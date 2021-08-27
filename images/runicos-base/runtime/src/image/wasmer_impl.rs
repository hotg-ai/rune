use std::{
    cell::Cell,
    collections::HashMap,
    convert::TryInto,
    sync::{Arc, Mutex},
};

use anyhow::{Context, Error};
use hotg_rune_core::{SerializableRecord, Shape, TFLITE_MIMETYPE};
use hotg_rune_runtime::{Capability, Image, Output};
use wasmer::{Array, Function, LazyInit, Memory, RuntimeError, ValueType, WasmPtr};
use hotg_rune_wasmer_runtime::Registrar;

use crate::{CapabilityFactory, Model, ModelFactory, OutputFactory};

use super::{BaseImage, Identifiers, LogFunc};

impl<'vm> Image<hotg_rune_wasmer_runtime::Registrar<'vm>> for BaseImage {
    fn initialize_imports(self, registrar: &mut Registrar<'vm>) {
        let BaseImage {
            capabilities,
            models,
            outputs,
            log,
        } = self;
        let identifiers = Identifiers::default();

        let log_env = LogEnv {
            log,
            memory: LazyInit::new(),
        };
        let cap_env = CapabilityEnv {
            factories: Arc::new(capabilities),
            instances: Arc::new(Mutex::new(HashMap::new())),
            identifiers: identifiers.clone(),
            memory: LazyInit::new(),
        };
        let model_env = ModelEnv {
            factories: Arc::new(models),
            instances: Arc::new(Mutex::new(HashMap::new())),
            identifiers: identifiers.clone(),
            memory: LazyInit::new(),
        };
        let output_env = OutputEnv {
            factories: Arc::new(outputs),
            instances: Arc::new(Mutex::new(HashMap::new())),
            identifiers,
            memory: LazyInit::new(),
        };

        let store = registrar.store();

        registrar
            .register_function(
                "env",
                "_debug",
                Function::new_native_with_env(store, log_env, debug),
            )
            .register_function(
                "env",
                "request_capability",
                Function::new_native_with_env(
                    store,
                    cap_env.clone(),
                    request_capability,
                ),
            )
            .register_function(
                "env",
                "request_capability_set_param",
                Function::new_native_with_env(
                    store,
                    cap_env.clone(),
                    request_capability_set_param,
                ),
            )
            .register_function(
                "env",
                "request_provider_response",
                Function::new_native_with_env(
                    store,
                    cap_env,
                    request_provider_response,
                ),
            )
            .register_function(
                "env",
                "tfm_model_invoke",
                Function::new_native_with_env(
                    store,
                    model_env.clone(),
                    tfm_model_invoke,
                ),
            )
            .register_function(
                "env",
                "tfm_preload_model",
                Function::new_native_with_env(
                    store,
                    model_env.clone(),
                    tfm_preload_model,
                ),
            )
            .register_function(
                "env",
                "rune_model_load",
                Function::new_native_with_env(
                    store,
                    model_env.clone(),
                    rune_model_load,
                ),
            )
            .register_function(
                "env",
                "rune_model_infer",
                Function::new_native_with_env(
                    store,
                    model_env,
                    rune_model_infer,
                ),
            )
            .register_function(
                "env",
                "request_output",
                Function::new_native_with_env(
                    store,
                    output_env.clone(),
                    request_output,
                ),
            )
            .register_function(
                "env",
                "consume_output",
                Function::new_native_with_env(
                    store,
                    output_env,
                    consume_output,
                ),
            );
    }
}

#[derive(Clone, wasmer::WasmerEnv)]
struct OutputEnv {
    factories: Arc<HashMap<u32, Box<dyn OutputFactory>>>,
    instances: Arc<Mutex<HashMap<u32, Box<dyn Output>>>>,
    identifiers: Identifiers,
    #[wasmer(export)]
    memory: LazyInit<Memory>,
}

fn request_output(
    env: &OutputEnv,
    output_type: u32,
) -> Result<u32, RuntimeError> {
    let factory = env
        .factories
        .get(&output_type)
        .with_context(|| match hotg_rune_core::outputs::name(output_type) {
            Some(name) => {
                format!("No handler registered for output \"{}\"", name)
            },
            None => format!("No handler registered for output {}", output_type),
        })
        .map_err(runtime_error)?;

    let output = factory
        .new_output(None)
        .context("Unable to instantiate the output")
        .map_err(runtime_error)?;

    let id = env.identifiers.next();
    env.instances.lock().unwrap().insert(id, output);

    Ok(id)
}

fn consume_output(
    env: &OutputEnv,
    output_id: u32,
    buffer: WasmPtr<u8, Array>,
    len: u32,
) -> Result<u32, RuntimeError> {
    let mut outputs = env.instances.lock().unwrap();
    let output = outputs
        .get_mut(&output_id)
        .with_context(|| format!("There is no output with ID {}", output_id))
        .map_err(runtime_error)?;

    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    let buffer = buffer
        .deref(memory, 0, len)
        .context("Invalid input")
        .map_err(runtime_error)?;

    // Safety: This function isn't reentrant so there are no concurrent
    // modifications. That also means it's safe to transmute [Cell<T>] to [T].
    let buffer = unsafe {
        std::slice::from_raw_parts(buffer.as_ptr() as *const u8, buffer.len())
    };

    output
        .consume(buffer)
        .context("Unable to consume the data")
        .map_err(runtime_error)?;

    Ok(len)
}

#[derive(Clone, wasmer::WasmerEnv)]
struct ModelEnv {
    factories: Arc<HashMap<String, Box<dyn ModelFactory>>>,
    instances: Arc<Mutex<HashMap<u32, Box<dyn Model>>>>,
    identifiers: Identifiers,
    #[wasmer(export)]
    memory: LazyInit<Memory>,
}

fn tfm_model_invoke(
    env: &ModelEnv,
    model_id: u32,
    input: WasmPtr<u8, Array>,
    input_len: u32,
    output: WasmPtr<u8, Array>,
    output_len: u32,
) -> Result<u32, RuntimeError> {
    let mut models = env.instances.lock().unwrap();
    let model = models
        .get_mut(&model_id)
        .with_context(|| format!("There is no model with ID {}", model_id))
        .map_err(runtime_error)?;

    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    let input = input
        .deref(memory, 0, input_len)
        .context("Invalid input")
        .map_err(runtime_error)?;

    let output = output
        .deref(memory, 0, output_len)
        .context("Invalid output")
        .map_err(runtime_error)?;

    // Safety: See safety comment from rune_model_infer()
    unsafe {
        model.infer(&[input], &[output]).map_err(runtime_error)?;
    }

    Ok(0)
}

fn rune_model_infer(
    env: &ModelEnv,
    model_id: u32,
    inputs: WasmPtr<WasmPtr<u8, Array>, Array>,
    outputs: WasmPtr<WasmPtr<u8, Array>, Array>,
) -> Result<u32, RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    let mut models = env.instances.lock().unwrap();
    let model = models
        .get_mut(&model_id)
        .with_context(|| format!("There is no model with ID {}", model_id))
        .map_err(runtime_error)?;

    let inputs = vector_of_tensors(memory, model.input_shapes(), inputs)
        .context("Invalid inputs")
        .map_err(runtime_error)?;
    let outputs = vector_of_tensors(memory, model.output_shapes(), outputs)
        .context("Invalid outputs")
        .map_err(runtime_error)?;

    // Safety: WebAssembly is single-threaded and this function isn't
    // re-entrant so we've got unique access to `output`.
    unsafe {
        model
            .infer(&inputs, &outputs)
            .context("Inference failed")
            .map_err(runtime_error)?;
    }

    Ok(0)
}

fn vector_of_tensors<'vm>(
    memory: &'vm Memory,
    shapes: &[Shape<'_>],
    ptr: WasmPtr<WasmPtr<u8, Array>, Array>,
) -> Result<Vec<&'vm [Cell<u8>]>, Error> {
    let pointers = ptr
        .deref(memory, 0, shapes.len() as u32)
        .context("Invalid tensor array pointer")?;

    let mut tensors = Vec::new();

    for (i, ptr) in pointers.iter().enumerate() {
        let ptr = ptr.get();
        let shape = &shapes[i];
        let data = ptr
            .deref(memory, 0, shape.size() as u32)
            .with_context(|| format!("Bad pointer for tensor {}", i))?;
        tensors.push(data);
    }

    Ok(tensors)
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct StringRef {
    data: WasmPtr<u8, Array>,
    len: u32,
}

// Safety: All bit patterns are valid and the wasmer memory will do any
// necessary bounds checks.
unsafe impl ValueType for StringRef {}

fn rune_model_load(
    env: &ModelEnv,
    mimetype: WasmPtr<u8, Array>,
    mimetype_len: u32,
    model: WasmPtr<u8, Array>,
    model_len: u32,
    input_descriptors: WasmPtr<StringRef, Array>,
    input_len: u32,
    output_descriptors: WasmPtr<StringRef, Array>,
    output_len: u32,
) -> Result<u32, RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    // Safety: This function isn't reentrant so there are no concurrent
    // modifications. That also means it's safe to transmute [Cell<T>] to [T].
    let (mimetype, model) = unsafe {
        let mimetype = mimetype
            .get_utf8_str(memory, mimetype_len)
            .context("Invalid mimtype string")
            .map_err(runtime_error)?;

        let model = model
            .deref(memory, 0, model_len)
            .context("Invalid model")
            .map_err(runtime_error)?;
        let model = std::slice::from_raw_parts(
            model.as_ptr() as *const u8,
            model.len(),
        );

        (mimetype, model)
    };

    let factory = env
        .factories
        .get(mimetype)
        .with_context(|| {
            format!(
                "No handlers registered for the \"{}\" model type",
                mimetype
            )
        })
        .map_err(runtime_error)?;

    let (inputs, outputs) = unsafe {
        let inputs =
            shape_from_descriptors(memory, input_descriptors, input_len)
                .map_err(runtime_error)?;
        let outputs =
            shape_from_descriptors(memory, output_descriptors, output_len)
                .map_err(runtime_error)?;

        (inputs, outputs)
    };

    let model = factory
        .new_model(model, Some(inputs.as_slice()), Some(outputs.as_slice()))
        .context("Unable to load the model")
        .map_err(runtime_error)?;

    let id = env.identifiers.next();
    env.instances.lock().unwrap().insert(id, model);

    Ok(id)
}

fn tfm_preload_model(
    env: &ModelEnv,
    model: WasmPtr<u8, Array>,
    model_len: u32,
    _: u32,
    _: u32,
) -> Result<u32, RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    // Safety: This function isn't reentrant so there are no concurrent
    // modifications. That also means it's safe to transmute [Cell<T>] to [T].
    let model = unsafe {
        let model = model
            .deref(memory, 0, model_len)
            .context("Invalid model")
            .map_err(runtime_error)?;
        std::slice::from_raw_parts(model.as_ptr() as *const u8, model.len())
    };

    let factory = env
        .factories
        .get(TFLITE_MIMETYPE)
        .with_context(|| {
            format!(
                "No handlers registered for the \"{}\" model type",
                TFLITE_MIMETYPE
            )
        })
        .map_err(runtime_error)?;

    let model = factory
        .new_model(model, None, None)
        .context("Unable to instantiate the model")
        .map_err(runtime_error)?;

    let id = env.identifiers.next();
    env.instances.lock().unwrap().insert(id, model);

    Ok(id)
}

/// # Safety
unsafe fn shape_from_descriptors(
    memory: &Memory,
    descriptors: WasmPtr<StringRef, Array>,
    len: u32,
) -> Result<Vec<Shape<'static>>, Error> {
    let descriptors = descriptors
        .deref(memory, 0, len)
        .context("Invalid descriptor pointer")?;

    let mut shapes = Vec::new();

    for (i, descriptor) in descriptors.iter().enumerate() {
        let StringRef { data, len } = descriptor.get();
        let descriptor = data.get_utf8_str(memory, len).with_context(|| {
            format!("The {}'th descriptor pointer is invalid", i)
        })?;
        let shape = descriptor.parse().with_context(|| {
            format!("Unable to parse the {}'th descriptor", i)
        })?;
        shapes.push(shape);
    }

    Ok(shapes)
}

#[derive(Clone, wasmer::WasmerEnv)]
struct CapabilityEnv {
    factories: Arc<HashMap<u32, Box<dyn CapabilityFactory>>>,
    instances: Arc<Mutex<HashMap<u32, Box<dyn Capability>>>>,
    identifiers: Identifiers,
    #[wasmer(export)]
    memory: LazyInit<Memory>,
}

fn request_capability(
    env: &CapabilityEnv,
    capability_type: u32,
) -> Result<u32, RuntimeError> {
    match env.factories.get(&capability_type) {
        Some(f) => {
            let cap = f
                .new_capability()
                .with_context(|| {
                    match hotg_rune_core::capabilities::name(capability_type) {
                        Some(n) => {
                            format!("Unable to create the \"{}\" capability", n)
                        },
                        None => format!(
                            "Unable to create capability type {}",
                            capability_type
                        ),
                    }
                })
                .map_err(runtime_error)?;
            let id = env.identifiers.next();
            env.instances.lock().unwrap().insert(id, cap);
            Ok(id)
        },
        None => match hotg_rune_core::capabilities::name(capability_type) {
            Some(name) => {
                return Err(runtime_error(anyhow::anyhow!(
                    "No \"{}\" capability registered",
                    name
                )));
            },
            None => Err(runtime_error(anyhow::anyhow!(
                "No capability registered for capability type {}",
                capability_type
            ))),
        },
    }
}

fn request_capability_set_param(
    env: &CapabilityEnv,
    capability_id: u32,
    key_ptr: WasmPtr<u8, Array>,
    key_len: u32,
    value_ptr: WasmPtr<u8, Array>,
    value_len: u32,
    value_type: u32,
) -> Result<u32, RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    // Safety: this function isn't reentrant, so we don't need to worry about
    // concurrent mutations.
    unsafe {
        let key = key_ptr
            .get_utf8_str(memory, key_len)
            .context("Unable to read the key")
            .map_err(runtime_error)?;

        let ty = value_type
            .try_into()
            .map_err(|()| Error::msg("Invalid key type"))
            .map_err(runtime_error)?;

        let value = value_ptr
            .deref(memory, 0, value_len)
            .context("Unable to read the value")
            .map_err(runtime_error)?;

        // Safety: this is sound when there are no concurrent modifications
        let value: &[u8] =
            std::slice::from_raw_parts(value.as_ptr().cast(), value.len());
        let value = hotg_rune_core::Value::from_le_bytes(ty, value)
            .context("Invalid value")
            .map_err(runtime_error)?;

        env.instances
            .lock()
            .unwrap()
            .get_mut(&capability_id)
            .context("No such capability")
            .map_err(runtime_error)?
            .set_parameter(key, value)
            .with_context(|| {
                format!(
                    "Unable to set the \"{}\" parameter to \"{}\"",
                    key, value
                )
            })
            .map_err(runtime_error)?;
    }

    Ok(0)
}

fn request_provider_response(
    env: &CapabilityEnv,
    buffer: WasmPtr<u8, Array>,
    len: u32,
    capability_id: u32,
) -> Result<u32, RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    // Safety: this function isn't reentrant, so we don't need to worry about
    // concurrent mutations.
    let buffer = unsafe {
        let buffer = buffer
            .deref_mut(memory, 0, len)
            .context("Invalid buffer pointer")
            .map_err(runtime_error)?;

        std::slice::from_raw_parts_mut(
            buffer.as_mut_ptr() as *mut u8,
            buffer.len(),
        )
    };

    env.instances
        .lock()
        .unwrap()
        .get_mut(&capability_id)
        .context("No such capability")
        .and_then(|c| c.generate(buffer))
        .map_err(runtime_error)?;

    Ok(buffer.len() as u32)
}

#[derive(Clone, wasmer::WasmerEnv)]
struct LogEnv {
    log: Arc<LogFunc>,
    #[wasmer(export)]
    memory: LazyInit<Memory>,
}

fn debug(
    env: &LogEnv,
    msg: WasmPtr<u8, Array>,
    len: u32,
) -> Result<u32, RuntimeError> {
    let memory = env
        .memory
        .get_ref()
        .context("The memory isn't initialized")
        .map_err(runtime_error)?;

    // Safety: this function isn't reentrant, so we don't need to worry about
    // concurrent mutations.
    unsafe {
        let message = msg
            .get_utf8_str(memory, len)
            .context("Unable to read the message")
            .map_err(runtime_error)?;

        log::debug!("Received message: {}", message);

        match serde_json::from_str::<SerializableRecord>(message) {
            Ok(record) => {
                record
                    .with_record(|r| (env.log)(r))
                    .map_err(runtime_error)?;
            },
            Err(e) => {
                log::warn!(
                    "Unable to deserialize {:?} as a log message: {}",
                    message,
                    e
                );
            },
        }
    }

    Ok(0)
}

fn runtime_error(e: Error) -> RuntimeError {
    RuntimeError::from_trap(wasmer_vm::Trap::User(e.into()))
}

#[cfg(test)]
mod tests {
    use syn::{ForeignItem, ForeignItemFn, Item};
    use wasmer::{Export, Store};
    use hotg_rune_wasmer_runtime::Registrar;
    use super::*;

    fn extern_functions(src: &str) -> impl Iterator<Item = ForeignItemFn> {
        let module: syn::File = syn::parse_str(src).unwrap();

        module
            .items
            .into_iter()
            .filter_map(|it| match it {
                Item::ForeignMod(e) => Some(e.items.into_iter()),
                _ => None,
            })
            .flatten()
            .filter_map(|item| match item {
                ForeignItem::Fn(f) => Some(f),
                _ => None,
            })
    }

    #[test]
    fn all_intrinsics_are_registered() {
        let store = Store::default();
        let intrinsics_rs = include_str!("../../../wasm/src/intrinsics.rs");
        let intrinsics = extern_functions(intrinsics_rs).map(|f| f.sig);
        let mut registrar = Registrar::new(&store);

        BaseImage::default().initialize_imports(&mut registrar);

        let imports = registrar.into_import_object();

        for intrinsic in intrinsics {
            let name = intrinsic.ident.to_string();
            let got = imports.get_export("env", &name).expect(&name);

            let got = match got {
                Export::Function(f) => f,
                other => panic!("\"{}\" was a {:?}", name, other),
            };
            let host_function_signature = &got.vm_function.signature;
            assert_eq!(
                intrinsic.inputs.len(),
                host_function_signature.params().len(),
                "parameters for \"{}\" are mismatched",
                name,
            );
        }
    }
}

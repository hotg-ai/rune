use std::{
    cell::Cell,
    collections::HashMap,
    convert::TryInto,
    fmt,
    marker::PhantomData,
    mem, slice,
    str::Utf8Error,
    sync::{Arc, Mutex},
};
use anyhow::{Context, Error};
use hotg_rune_core::{SerializableRecord, Shape, TFLITE_MIMETYPE};
use hotg_rune_runtime::{Capability, Image, Output};
use wasm3::{CallContext, error::Trap};
use hotg_rune_wasm3_runtime::Registrar;
use crate::{
    CapabilityFactory, Model, ModelFactory, OutputFactory,
    image::{BaseImage, Identifiers, LogFunc},
};

/// Extends the `wasm3::CallContext` struct with memory access helpers.
trait CallContextExt<'cc> {
    fn get_slice(&self, start: u32, len: u32) -> &'cc [u8];
    unsafe fn get_mut_slice(&mut self, start: u32, len: u32) -> &'cc mut [u8];
}

impl<'cc> CallContextExt<'cc> for CallContext<'cc> {
    fn get_slice(&self, start: u32, len: u32) -> &'cc [u8] {
        // Safety: this module never calls WASM functions itself.
        let memory = unsafe { &*self.memory() };
        &memory[start as usize..][..len as usize]
    }

    unsafe fn get_mut_slice(&mut self, start: u32, len: u32) -> &'cc mut [u8] {
        let memory = &mut *self.memory_mut();
        &mut memory[start as usize..][..len as usize]
    }
}

struct WasmPtr<ELEM, MODE> {
    offset: u32,
    _p: PhantomData<(ELEM, MODE)>,
}

impl<ELEM, MODE> fmt::Debug for WasmPtr<ELEM, MODE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WasmPtr")
            .field("offset", &self.offset)
            .finish()
    }
}

impl<ELEM, MODE> PartialEq for WasmPtr<ELEM, MODE> {
    fn eq(&self, other: &Self) -> bool { self.offset == other.offset }
}

impl<ELEM, MODE> Clone for WasmPtr<ELEM, MODE> {
    fn clone(&self) -> Self { *self }
}

impl<ELEM, MODE> Copy for WasmPtr<ELEM, MODE> {}

// There's no way to implement `WasmType` for user-defined types, so we'll have
// to make due with accepting `u32` and converting that to `WasmPtr` via
// `.into()`.
impl<ELEM, MODE> From<u32> for WasmPtr<ELEM, MODE> {
    fn from(offset: u32) -> Self {
        Self {
            offset,
            _p: PhantomData,
        }
    }
}

struct Array;

impl<T> WasmPtr<T, Array> {
    fn deref<'cc>(
        self,
        cc: &CallContext<'cc>,
        index: u32,
        length: u32,
    ) -> Option<&'cc [Cell<T>]> {
        let elem_size = mem::size_of::<T>();

        // Semantics with ZSTs are unclear, so reject this.
        assert!(elem_size > 0, "ZST in `WasmPtr::deref`");

        let start_offset = index as usize * elem_size;
        let len_bytes = length as usize * elem_size;

        let bytes = cc.get_slice(
            (self.offset as usize)
                .checked_add(start_offset)?
                .try_into()
                .ok()?,
            len_bytes.try_into().ok()?,
        );
        if bytes.as_ptr() as usize % mem::align_of::<T>() != 0 {
            // `self` was unaligned
            return None;
        }

        Some(unsafe {
            slice::from_raw_parts(bytes.as_ptr() as _, length as usize)
        })
    }

    /// # Safety
    ///
    /// Exclusivity of the returned reference must be upheld by the caller.
    unsafe fn deref_mut<'cc>(
        self,
        cc: &mut CallContext<'cc>,
        index: u32,
        length: u32,
    ) -> Option<&'cc mut [Cell<T>]> {
        let elem_size = mem::size_of::<T>();

        // Semantics with ZSTs are unclear, so reject this.
        assert!(elem_size > 0, "ZST in `WasmPtr::deref`");

        let start_offset = index as usize * elem_size;
        let len_bytes = length as usize * elem_size;

        let bytes = cc.get_mut_slice(
            (self.offset as usize)
                .checked_add(start_offset)?
                .try_into()
                .ok()?,
            len_bytes.try_into().ok()?,
        );
        if bytes.as_ptr() as usize % mem::align_of::<T>() != 0 {
            // `self` was unaligned
            return None;
        }

        Some(slice::from_raw_parts_mut(
            bytes.as_mut_ptr() as _,
            length as usize,
        ))
    }
}

impl WasmPtr<u8, Array> {
    fn get_utf8_str<'a>(
        self,
        cc: &CallContext<'a>,
        len: u32,
    ) -> Result<&'a str, Utf8Error> {
        let bytes = cc.get_slice(self.offset, len);
        std::str::from_utf8(bytes)
    }
}

struct RuntimeError(anyhow::Error);

fn res(res: Result<u32, RuntimeError>) -> Result<u32, Trap> {
    res.map_err(|e| {
        log::error!("{:?}", e.0);
        Trap::Abort
    })
}

macro_rules! hostfn_wrappers {
    (
        $(
            $hostfn_name:ident: ( $( $name:ident, )* );
        )+
    ) => {
        $(
            #[allow(dead_code, non_snake_case)]
            fn $hostfn_name<Env $(, $name)*>(
                f: fn(CallContext<'_>, &Env $(, $name)*) -> Result<u32, RuntimeError>,
                env: Env,
            ) -> impl FnMut(CallContext<'_>, ($($name,)*)) -> Result<u32, Trap> {
                move |cc, ($($name,)*)| res(f(cc, &env $(, $name)*))
            }
        )+
    };
}

hostfn_wrappers! {
    hostfn0: ();
    hostfn1: (A0,);
    hostfn2: (A0, A1,);
    hostfn3: (A0, A1, A2,);
    hostfn4: (A0, A1, A2, A3,);
    hostfn5: (A0, A1, A2, A3, A4,);
    hostfn6: (A0, A1, A2, A3, A4, A5,);
    hostfn7: (A0, A1, A2, A3, A4, A5, A6,);
    hostfn8: (A0, A1, A2, A3, A4, A5, A6, A7,);
}

impl<'vm> Image<hotg_rune_wasm3_runtime::Registrar<'vm>> for BaseImage {
    fn initialize_imports(self, registrar: &mut Registrar<'vm>) {
        let BaseImage {
            capabilities,
            models,
            outputs,
            log,
        } = self;
        let identifiers = Identifiers::default();

        let log_env = LogEnv { log };
        let cap_env = CapabilityEnv {
            factories: Arc::new(capabilities),
            instances: Arc::new(Mutex::new(HashMap::new())),
            identifiers: identifiers.clone(),
        };
        let model_env = ModelEnv {
            factories: Arc::new(models),
            instances: Arc::new(Mutex::new(HashMap::new())),
            identifiers: identifiers.clone(),
        };
        let output_env = OutputEnv {
            factories: Arc::new(outputs),
            instances: Arc::new(Mutex::new(HashMap::new())),
            identifiers,
        };

        registrar
            .register_function("env", "_debug", hostfn2(debug, log_env))
            .register_function(
                "env",
                "request_capability",
                hostfn1(request_capability, cap_env.clone()),
            )
            .register_function(
                "env",
                "request_capability_set_param",
                hostfn6(request_capability_set_param, cap_env.clone()),
            )
            .register_function(
                "env",
                "request_provider_response",
                hostfn3(request_provider_response, cap_env),
            )
            .register_function(
                "env",
                "tfm_model_invoke",
                hostfn5(tfm_model_invoke, model_env.clone()),
            )
            .register_function(
                "env",
                "tfm_preload_model",
                hostfn4(tfm_preload_model, model_env.clone()),
            )
            .register_function(
                "env",
                "rune_model_load",
                hostfn8(rune_model_load, model_env.clone()),
            )
            .register_function(
                "env",
                "rune_model_infer",
                hostfn3(rune_model_infer, model_env),
            )
            .register_function(
                "env",
                "request_output",
                hostfn1(request_output, output_env.clone()),
            )
            .register_function(
                "env",
                "consume_output",
                hostfn3(consume_output, output_env),
            );
    }
}

#[derive(Clone)]
struct OutputEnv {
    factories: Arc<HashMap<u32, Box<dyn OutputFactory>>>,
    instances: Arc<Mutex<HashMap<u32, Box<dyn Output>>>>,
    identifiers: Identifiers,
}

fn request_output(
    _cc: CallContext<'_>,
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
    cc: CallContext<'_>,
    env: &OutputEnv,
    output_id: u32,
    buffer: u32,
    len: u32,
) -> Result<u32, RuntimeError> {
    let buffer: WasmPtr<u8, Array> = buffer.into();

    let mut outputs = env.instances.lock().unwrap();
    let output = outputs
        .get_mut(&output_id)
        .with_context(|| format!("There is no output with ID {}", output_id))
        .map_err(runtime_error)?;

    let buffer = buffer
        .deref(&cc, 0, len)
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

#[derive(Clone)]
struct ModelEnv {
    factories: Arc<HashMap<String, Box<dyn ModelFactory>>>,
    instances: Arc<Mutex<HashMap<u32, Box<dyn Model>>>>,
    identifiers: Identifiers,
}

fn tfm_model_invoke(
    cc: CallContext<'_>,
    env: &ModelEnv,
    model_id: u32,
    input: u32,
    input_len: u32,
    output: u32,
    output_len: u32,
) -> Result<u32, RuntimeError> {
    let input: WasmPtr<u8, Array> = input.into();
    let output: WasmPtr<u8, Array> = output.into();

    let mut models = env.instances.lock().unwrap();
    let model = models
        .get_mut(&model_id)
        .with_context(|| format!("There is no model with ID {}", model_id))
        .map_err(runtime_error)?;

    let input = input
        .deref(&cc, 0, input_len)
        .context("Invalid input")
        .map_err(runtime_error)?;

    let output = output
        .deref(&cc, 0, output_len)
        .context("Invalid output")
        .map_err(runtime_error)?;

    // Safety: See safety comment from rune_model_infer()
    unsafe {
        model.infer(&[input], &[output]).map_err(runtime_error)?;
    }

    Ok(0)
}

fn rune_model_infer(
    cc: CallContext<'_>,
    env: &ModelEnv,
    model_id: u32,
    inputs: u32,
    outputs: u32,
) -> Result<u32, RuntimeError> {
    let inputs: WasmPtr<WasmPtr<u8, Array>, Array> = inputs.into();
    let outputs: WasmPtr<WasmPtr<u8, Array>, Array> = outputs.into();

    let mut models = env.instances.lock().unwrap();
    let model = models
        .get_mut(&model_id)
        .with_context(|| format!("There is no model with ID {}", model_id))
        .map_err(runtime_error)?;

    let inputs = vector_of_tensors(&cc, model.input_shapes(), inputs)
        .context("Invalid inputs")
        .map_err(runtime_error)?;
    let outputs = vector_of_tensors(&cc, model.output_shapes(), outputs)
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

fn vector_of_tensors<'cc>(
    cc: &CallContext<'cc>,
    shapes: &[Shape<'_>],
    ptr: WasmPtr<WasmPtr<u8, Array>, Array>,
) -> Result<Vec<&'cc [Cell<u8>]>, Error> {
    let pointers = ptr
        .deref(cc, 0, shapes.len() as u32)
        .context("Invalid tensor array pointer")?;

    let mut tensors = Vec::new();

    for (i, ptr) in pointers.iter().enumerate() {
        let ptr = ptr.get();
        let shape = &shapes[i];
        let data = ptr
            .deref(cc, 0, shape.size() as u32)
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

fn rune_model_load(
    cc: CallContext<'_>,
    env: &ModelEnv,
    mimetype: u32,
    mimetype_len: u32,
    model: u32,
    model_len: u32,
    input_descriptors: u32,
    input_len: u32,
    output_descriptors: u32,
    output_len: u32,
) -> Result<u32, RuntimeError> {
    let mimetype: WasmPtr<u8, Array> = mimetype.into();
    let model: WasmPtr<u8, Array> = model.into();
    let input_descriptors: WasmPtr<StringRef, Array> = input_descriptors.into();
    let output_descriptors: WasmPtr<StringRef, Array> =
        output_descriptors.into();

    // Safety: This function isn't reentrant so there are no concurrent
    // modifications. That also means it's safe to transmute [Cell<T>] to [T].
    let (mimetype, model) = unsafe {
        let mimetype = mimetype
            .get_utf8_str(&cc, mimetype_len)
            .context("Invalid mimtype string")
            .map_err(runtime_error)?;

        let model = model
            .deref(&cc, 0, model_len)
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
        let inputs = shape_from_descriptors(&cc, input_descriptors, input_len)
            .map_err(runtime_error)?;
        let outputs =
            shape_from_descriptors(&cc, output_descriptors, output_len)
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
    cc: CallContext<'_>,
    env: &ModelEnv,
    model: u32,
    model_len: u32,
    _: u32,
    _: u32,
) -> Result<u32, RuntimeError> {
    let model: WasmPtr<u8, Array> = model.into();

    // Safety: This function isn't reentrant so there are no concurrent
    // modifications. That also means it's safe to transmute [Cell<T>] to [T].
    let model = unsafe {
        let model = model
            .deref(&cc, 0, model_len)
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

// # Safety
unsafe fn shape_from_descriptors<'cc>(
    cc: &CallContext<'cc>,
    descriptors: WasmPtr<StringRef, Array>,
    len: u32,
) -> Result<Vec<Shape<'static>>, Error> {
    let descriptors = descriptors
        .deref(cc, 0, len)
        .context("Invalid descriptor pointer")?;

    let mut shapes = Vec::new();

    for (i, descriptor) in descriptors.iter().enumerate() {
        let StringRef { data, len } = descriptor.get();
        let descriptor = data.get_utf8_str(cc, len).with_context(|| {
            format!("The {}'th descriptor pointer is invalid", i)
        })?;
        let shape = descriptor.parse().with_context(|| {
            format!("Unable to parse the {}'th descriptor", i)
        })?;
        shapes.push(shape);
    }

    Ok(shapes)
}

#[derive(Clone)]
struct CapabilityEnv {
    factories: Arc<HashMap<u32, Box<dyn CapabilityFactory>>>,
    instances: Arc<Mutex<HashMap<u32, Box<dyn Capability>>>>,
    identifiers: Identifiers,
}

fn request_capability(
    _cc: CallContext<'_>,
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
    cc: CallContext<'_>,
    env: &CapabilityEnv,
    capability_id: u32,
    key_ptr: u32,
    key_len: u32,
    value_ptr: u32,
    value_len: u32,
    value_type: u32,
) -> Result<u32, RuntimeError> {
    let key_ptr: WasmPtr<u8, Array> = key_ptr.into();
    let value_ptr: WasmPtr<u8, Array> = value_ptr.into();

    // Safety: this function isn't reentrant, so we don't need to worry about
    // concurrent mutations.
    unsafe {
        let key = key_ptr
            .get_utf8_str(&cc, key_len)
            .context("Unable to read the key")
            .map_err(runtime_error)?;

        let ty = value_type
            .try_into()
            .map_err(|()| Error::msg("Invalid key type"))
            .map_err(runtime_error)?;

        let value = value_ptr
            .deref(&cc, 0, value_len)
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
    mut cc: CallContext<'_>,
    env: &CapabilityEnv,
    buffer: u32,
    len: u32,
    capability_id: u32,
) -> Result<u32, RuntimeError> {
    let buffer: WasmPtr<u8, Array> = buffer.into();

    // Safety: this function isn't reentrant, so we don't need to worry about
    // concurrent mutations.
    let buffer = unsafe {
        let buffer = buffer
            .deref_mut(&mut cc, 0, len)
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

#[derive(Clone)]
struct LogEnv {
    log: Arc<LogFunc>,
}

fn debug(
    cc: CallContext<'_>,
    env: &LogEnv,
    msg: u32,
    len: u32,
) -> Result<u32, RuntimeError> {
    let msg: WasmPtr<u8, Array> = msg.into();

    let message = msg
        .get_utf8_str(&cc, len)
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

    Ok(0)
}

fn runtime_error(e: Error) -> RuntimeError { RuntimeError(e) }

#[cfg(test)]
#[cfg(never)] // TODO: port this over (wasm3 lacks reflection though)
mod tests {
    use syn::{ForeignItem, ForeignItemFn, Item};
    use hotg_rune_wasm3_runtime::Registrar;
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

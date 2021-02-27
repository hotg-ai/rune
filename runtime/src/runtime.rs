use anyhow::Error;
use rand::RngCore;
use std::{ffi::c_void, sync::Mutex};
use wasmer_runtime::{
    error::{Error as WasmerError, RuntimeError, RuntimeResult},
    Array, Ctx, ImportObject, Instance, WasmPtr,
};

pub struct Runtime {
    instance: Instance,
}

impl Runtime {
    pub fn load<E>(rune: &[u8], env: E) -> Result<Self, WasmerError>
    where
        E: Environment + Send + Sync + 'static,
    {
        let module = wasmer_runtime::compile(rune)?;
        let imports = onetime_import_object(env);
        let instance = module.instantiate(&imports)?;

        Ok(Runtime { instance })
    }
}

fn onetime_import_object<E>(env: E) -> ImportObject
where
    E: Environment + Send + Sync + 'static,
{
    let env = Mutex::new(Some(env));

    import_object(move || {
        env.lock()
            .unwrap()
            .take()
            .expect("Initializer should only ever be called once")
    })
}

/// Create a new [`ImportObject`] using a closure that will instantiate a new
/// [`Environment`] for every new WebAssembly [`Instance`].
fn import_object<F, E>(constructor: F) -> ImportObject
where
    F: Fn() -> E + Send + Sync + 'static,
    E: Environment,
{
    wasmer_runtime::imports! {
        move || {
            let env = constructor();

            let ctx = Box::new(Context { env });
            let free = |data: *mut c_void| unsafe {
                let _ = Box::from_raw(data as *mut Context<E>);
            };

            (Box::into_raw(ctx) as *mut c_void, free)
        },

        "env" => {
            "_debug" => wasmer_runtime::func!(log::<E>),
        },
    }
}

/// Contextual state associated with a single instance of the [`Runtime`].
struct Context<E> {
    env: E,
}

fn log<E>(
    ctx: &mut Ctx,
    buffer: WasmPtr<u8, Array>,
    len: u32,
) -> RuntimeResult<u32>
where
    E: Environment,
{
    let (mem, context) = unsafe { ctx.memory_and_data_mut::<Context<E>>(0) };

    match buffer.get_utf8_string(mem, len) {
        Some(msg) => {
            context.env.log(msg);
            // FIXME: We should just return () here because logging isn't
            // fallible.
            Ok(0)
        },
        None => Err(RuntimeError::User(Box::new(Error::msg("")))),
    }
}

pub trait Environment: 'static {
    fn rng(&mut self) -> Option<&mut dyn RngCore> { None }

    fn log(&mut self, _msg: &str) {}
}

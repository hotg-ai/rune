use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    },
};

use log::{Level, Record};
use rune_runtime::{CallContext, Function, Image, Output, Registrar};
use anyhow::{Context, Error};
use runic_types::SerializableRecord;

type OutputFactory =
    dyn Fn() -> Result<Box<dyn Output>, Error> + Send + Sync + 'static;

#[derive(Clone)]
pub struct BaseImage {
    serial: Arc<OutputFactory>,
}

impl Image for BaseImage {
    fn initialize_imports(self, registrar: &mut dyn Registrar) {
        let ids = Identifiers::default();
        let outputs = Arc::new(Mutex::new(HashMap::new()));

        registrar.register_function("env", "_debug", Function::new(log));

        let output_factories = Outputs {
            serial: Arc::clone(&self.serial),
        };
        registrar.register_function(
            "env",
            "request_output",
            request_output(&ids, &outputs, output_factories),
        );
        registrar.register_function(
            "env",
            "consume_output",
            consume_output(&outputs),
        );
    }
}

fn consume_output(
    outputs: &Arc<Mutex<HashMap<u32, Box<dyn Output>>>>,
) -> Function {
    let outputs = Arc::clone(outputs);

    Function::new(
        move |ctx: &dyn CallContext, (id, address, len): (u32, u32, u32)| {
            let data = ctx.memory(address, len).context("Bad input pointer")?;
            let mut outputs = outputs.lock().unwrap();
            let output = outputs.get_mut(&id).context("Invalid output")?;
            output.consume(data)
        },
    )
}

struct Outputs {
    serial: Arc<OutputFactory>,
}

fn log(ctx: &dyn CallContext, (msg, len): (u32, u32)) -> Result<(), Error> {
    let msg = ctx.utf8_str(msg, len)?;

    // this is a little more verbose than normal because we want to try
    // *really* hard to log messages and abort on errors.
    match serde_json::from_str::<SerializableRecord>(msg) {
        Ok(r) => {
            r.with_record(|record| log::logger().log(record));

            if r.level == Level::Error {
                // Make sure we abort on error
                return Err(Error::msg(r.message.into_owned()));
            }
        },
        Err(_) => log::logger()
            .log(&Record::builder().args(format_args!("{}", msg)).build()),
    };

    todo!()
}

#[derive(Default, Debug, Clone)]
struct Identifiers(Arc<AtomicU32>);

impl Identifiers {
    fn next(&self) -> u32 { self.0.fetch_add(1, Ordering::SeqCst) }
}

fn request_output(
    ids: &Identifiers,
    outputs: &Arc<Mutex<HashMap<u32, Box<dyn Output>>>>,
    constructors: Outputs,
) -> Function {
    let ids = ids.clone();
    let outputs = Arc::clone(outputs);

    Function::new(move |_: &dyn CallContext, (output_type,): (u32,)| {
        let output = match output_type {
            runic_types::outputs::SERIAL => (constructors.serial)()
                .context("Unable to create a new SERIAL output")?,
            _ => anyhow::bail!("Unknown output type: {}", output_type),
        };

        let id = ids.next();
        log::debug!("Output {} = {:?}", id, output);
        outputs.lock().unwrap().insert(id, output);

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use std::cell::UnsafeCell;

    use rune_runtime::WasmValue;

    use super::*;

    #[derive(Default, Debug)]
    struct DummyOutput(Arc<AtomicU32>);

    impl rune_runtime::Output for DummyOutput {
        fn consume(&mut self, _: &[u8]) -> Result<(), Error> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[derive(Default, Debug)]
    struct DummyCallContext {
        memory: UnsafeCell<Vec<u8>>,
    }

    impl CallContext for DummyCallContext {
        fn memory(&self, address: u32, len: u32) -> Result<&[u8], Error> {
            let start = address as usize;
            let end = start + len as usize;
            let memory = unsafe { &*self.memory.get() };
            memory.get(start..end).context("Out of bounds")
        }

        unsafe fn memory_mut(
            &self,
            address: u32,
            len: u32,
        ) -> Result<&mut [u8], Error> {
            let start = address as usize;
            let end = start + len as usize;
            let memory = &mut *self.memory.get();
            memory.get_mut(start..end).context("Out of bounds")
        }
    }

    #[test]
    fn invoke_output() {
        let calls = Arc::new(AtomicU32::new(0));
        let ctx = DummyCallContext::default();
        let d = DummyOutput(Arc::clone(&calls));
        let mut outputs = HashMap::new();
        outputs.insert(3_u32, Box::new(d) as Box<dyn Output>);
        let outputs = Arc::new(Mutex::new(outputs));
        let func = consume_output(&outputs);

        let ret = func
            .call(
                &ctx,
                &[WasmValue::I32(3), WasmValue::I32(0), WasmValue::I32(0)],
            )
            .unwrap();

        assert!(ret.is_empty());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}

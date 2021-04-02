use log::{Level, Record};
use rune_runtime::{CallContext, Function, Image, Registrar};
use anyhow::Error;
use runic_types::SerializableRecord;

pub struct BaseImage {}

impl Image for BaseImage {
    fn initialize_imports(self, registrar: &mut dyn Registrar) {
        registrar.register_function("env", "_debug", Function::new(log));
    }
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

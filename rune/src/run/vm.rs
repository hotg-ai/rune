use log;
use runic_types::*;

use wasmer_runtime::{func, imports, Array, Ctx, Instance, WasmPtr};

use tflite::ops::builtin::BuiltinOpResolver;
use tflite::{FlatBufferModel, InterpreterBuilder};


/// Rune Executor 
///  Executes the Rune and provides the appropriate interfaces
pub struct VM {

}

///
impl VM {
    pub fn init(filename: &str) -> VM {
        log::info!("Initializing");

        let rune_bytes = match std::fs::read(filename) {
            Ok(res) => res,
            Err(_err) => { 
                log::error!("Failed to load container {}", filename);
                std::process::exit(1);
            }
        };

        return VM{};
    }
}
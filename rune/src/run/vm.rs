use log;

mod imports;

use wasmer_runtime::{instantiate, Func, Instance, Array, WasmPtr};


/// Rune Executor 
///  Executes the Rune and provides the appropriate interfaces
pub struct VM {
    instance: Instance
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
        let imports = imports::get_imports();
        let instance = instantiate(&rune_bytes[..], &imports).expect("failed to instantiate Rune");

        let manifest: Func<(), u32> = instance.exports.get("_manifest").unwrap();

        let manifest_size: u32 = manifest.call().expect("failed to call manifest");

        return VM{ instance };
    }

    pub fn call(&self, input: Vec<u8>) -> Vec<u8> {
        let instance = &self.instance;
        let get_provider_response_buffer_pointer: Func<(), WasmPtr<u8, Array>> = instance
            .exports
            .get("provider_response_ptr")
            .expect("provider_response_ptr");
    
        let pr_ptr = get_provider_response_buffer_pointer.call().unwrap();
    
        let wasm_instance_memory = instance.context().memory(0);
        log::debug!("Trying to write provider response");
        let len = input.len() as u32;
        let memory_writer = pr_ptr.deref(wasm_instance_memory, 0, len).unwrap();
    
        //Refactor THIS
        let mut idx = 0;
        for b in input.into_iter() {
            memory_writer[idx].set(b);
            idx = idx + 1;
        }
    
        let call_fn: Func<u32, i32> = instance.exports.get("_call").unwrap();
    
        let feature_buff_size = call_fn.call(len).expect("failed to _call");
        log::debug!("Guest::_call() returned {}", feature_buff_size);
    
        let feature_data_buf: Vec<u8> = vec![];
    
        return feature_data_buf;
    }
}
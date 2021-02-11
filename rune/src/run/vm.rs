use log;

use std::ffi::c_void;

use wasmer_runtime::{instantiate, Func, Instance, func, imports, Array, Ctx, WasmPtr};

pub mod provider;
pub mod capability;
use crate::run::vm::provider::*;




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

        log::debug!("Loaded {} bytes from {} container", rune_bytes.len(), filename);

        let mut provider = Provider::init();
        let imports = VM::get_imports();
        let mut instance = instantiate(&rune_bytes[..], &imports).expect("failed to instantiate Rune");
        instance.context_mut().data = &mut provider as *mut _ as *mut c_void;
        let manifest: Func<(), u32> = instance.exports.get("_manifest").unwrap();

        let manifest_size: u32 = manifest.call().expect("failed to call manifest");



        return VM{ instance };
    }


    pub fn get_imports() -> wasmer_runtime::ImportObject {
       
        let ims = imports! {
            "env" => {
                "tfm_model_invoke" => func!(tfm_model_invoke),
                "tfm_preload_model" => func!(tfm_preload_model),
                "_debug" => func!(_debug),
                "request_capability" => func!(request_capability),
                "request_capability_set_param" => func!(request_capability_set_param),
                "request_manifest_output" => func!(request_manifest_output),
                "request_provider_response" => func!(request_provider_response)
            },
        };

        return ims;
    }

    pub fn call(&self, input: Vec<u8>) -> Vec<u8> {
        let instance = &self.instance;
       
        log::info!("CALLING ");
        let call_fn: Func<(i32, i32, i32), i32> = instance.exports.get("_call").unwrap();
    
        let feature_buff_size = call_fn.call(runic_types::CAPABILITY::RAND as i32, runic_types::PARAM_TYPE::FLOAT as i32, 0).expect("failed to _call");
        log::debug!("Guest::_call() returned {}", feature_buff_size);
    
        let feature_data_buf: Vec<u8> = vec![0,2,1,2];
    
        return feature_data_buf;
    }
}




fn get_mem_str(ctx: &Ctx, ptr: WasmPtr<u8, Array>, data_len: u32) -> std::string::String {
    let str_vec = get_mem_array(ctx, ptr, data_len);
    let string = std::str::from_utf8(&str_vec).unwrap();
    return std::string::String::from(string);
}

fn get_mem_array(ctx: &Ctx, ptr: WasmPtr<u8, Array>, data_len: u32) -> Vec<u8> {
    let memory = ctx.memory(0);
    // let memory = ctx.memory(0);
   
    let str_bytes = match ptr.deref(memory, 0, data_len) {
        Some(m) => m,
        _ => panic!("Couldn't get model  bytes"),
    };
    let str_vec: Vec<std::cell::Cell<u8>> = str_bytes.iter().cloned().collect();

    let str_vec: Vec<u8> = str_vec.iter().map(|x| x.get()).collect();

    return str_vec;
}



pub fn tfm_preload_model(
    ctx: &mut Ctx,
    model_idx: WasmPtr<u8, Array>,
    model_len: u32,
    inputs: u32,
    outputs: u32
) -> u32 {
    let provider: &mut Provider = unsafe { &mut *(ctx.data as *mut Provider) };
    let model_bytes = get_mem_array(ctx, model_idx, model_len);
    

    return provider.add_model(model_bytes, inputs, outputs);
}

pub fn tfm_model_invoke(
    ctx: &mut Ctx,
    feature_idx: WasmPtr<u8, Array>,
    feature_len: u32,
) -> u32 {

    log::info!("Calling tfm_model_invoke");
    let feature_bytes = get_mem_array(ctx, feature_idx, feature_len);
    log::info!("{:?}", feature_bytes);
    let provider: &mut Provider = unsafe { &mut *(ctx.data as *mut Provider) };

    provider.predict_model::<f32>(0, feature_bytes.to_owned().to_vec(), runic_types::PARAM_TYPE::FLOAT);

    return 0;
}

pub fn _debug(ctx: &mut Ctx, ptr:  WasmPtr<u8, Array>, len: u32) -> u32 {

    log::info!("[Rune::Debug]{}", get_mem_str(ctx, ptr, len));
    return 0;
}

pub fn request_capability(ctx: &mut Ctx, ct: u32) -> u32 {
    let provider: &mut Provider = unsafe { &mut *(ctx.data as *mut Provider) };

    log::info!("Requesting Capability");
    return provider.request_capability(ct);

}

pub fn request_capability_set_param(ctx: &mut Ctx, idx:u32, key_str_ptr: WasmPtr<u8, Array>, key_str_len:u32, value_ptr: WasmPtr<u8, Array>, value_len:u32, value_type:u32) -> u32
{
    let provider: &mut Provider = unsafe { &mut *(ctx.data as *mut Provider) };
    log::info!("Setting param");
    let key_str = get_mem_str(ctx, key_str_ptr, key_str_len);

    let value = get_mem_array(ctx, value_ptr, value_len);

    

    provider.set_capability_request_param(idx, key_str.clone(), value.clone(), runic_types::PARAM_TYPE::from_u32(value_type));
 

    return 0;
}

pub fn request_manifest_output(ctx: &mut Ctx, t:u32) -> u32{
    log::info!("Setting output");
    return 0;
}

pub fn request_provider_response(
    ctx: &mut Ctx,
    provider_response_idx: WasmPtr<u8, Array>,
    max_allowed_provider_response: u32,
    capability_idx: u32
) -> u32 {

    let provider: &mut Provider = unsafe { &mut *(ctx.data as *mut Provider) };
    log::info!("Requesting provider response");

    // Get Capaability and get input 
    let input: Vec<u8> = f32::to_be_bytes(0.2).to_vec();

    let wasm_instance_memory = ctx.memory(0);
    log::debug!("Trying to write provider response");
    
    let len = input.len() as u32;
    let memory_writer = provider_response_idx.deref(wasm_instance_memory, 0, len).unwrap();

    //Refactor THIS
    let mut idx = 0;
    for b in input.into_iter() {
        memory_writer[idx].set(b);
        idx = idx + 1;
    }


    return len;
}


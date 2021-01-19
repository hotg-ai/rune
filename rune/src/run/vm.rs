use log;

use std::ffi::c_void;
use std::cell::RefCell;
use wasmer_runtime::{instantiate, Func, Instance, func, imports, Array, Ctx, WasmPtr};
use std::sync::{Arc, Mutex};


use crate::run::vm::capability::*;

pub struct Model {
    weights: Vec<u8>
}

pub struct Provider {
    requests: RefCell<Vec<RefCell<CapabilityRequest>>>,
    models: Vec<Model>

}

impl Provider {
    pub fn init() -> Provider{
        return Provider{
            requests: RefCell::new(Vec::new()),
            models: vec![]
        };
    }

    // pub fn request_capability(&mut self, requested: runic_types::CAPABILITY) -> u32 {
    //     let idx = self.requests.len() as u32;
    //     self.requests.push(CapabilityRequest::init(requested));
    //     return idx;
    // }


    pub fn request_capability(&mut self, requested: u32) -> u32 {
        
        let idx = self.requests.borrow().len() as u32;
        let mut cr = CapabilityRequest::init(runic_types::CAPABILITY::from_u32(requested));

        self.requests.borrow_mut().push(RefCell::new(cr));
        log::info!("Setting capability({}) {:?}", idx, runic_types::CAPABILITY::from_u32(requested));
        return idx;
    }



    pub fn set_capability_request_param(&mut self, request_idx: u32, key: String, value: Vec<u8>, value_t: runic_types::PARAM_TYPE) {
        let capability_request = match self.requests.borrow_mut().get(request_idx as usize)  {
            Some(cr) => {
                cr.borrow_mut().set_param(key, value, value_t);
                log::info!("Parameter set for Capability {:?}", cr.borrow());
            },
            _ => {
                log::warn!("Rune called to set param on capability_request({}) that does not exist", request_idx);
                return;
            }
        };


    }
}


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
       
       
        let call_fn: Func<(i32, i32, i32), i32> = instance.exports.get("_call").unwrap();
    
        let feature_buff_size = call_fn.call(input.len() as i32, 0i32, 0i32).expect("failed to _call");
        log::debug!("Guest::_call() returned {}", feature_buff_size);
    
        let feature_data_buf: Vec<u8> = vec![];
    
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
    log::info!("Calling tfm_preload_model");


    let model_bytes = get_mem_array(ctx, model_idx, model_len);

    log::info!("BYTES = {:?}", model_bytes);
 
    return 0;
}

pub fn tfm_model_invoke(
    ctx: &mut Ctx,
    feature_idx: WasmPtr<u8, Array>,
    feature_len: u32,
) -> u32 {

    log::info!("Calling tfm_model_invoke");

    // let memory = ctx.memory(0);

    // let model_bytes = match model_ptr.deref(memory, 0, model_len) {
    //     Some(m) => m,
    //     _ => panic!("Couldn't get model  bytes"),
    // };

    // let mut model_buf: Vec<u8> = vec![];
    // for m in model_bytes {
    //     model_buf.push(m.get())
    // }

    // let feature_bytes = match feature_ptr.deref(memory, 0, model_len) {
    //     Some(m) => m,
    //     _ => panic!("Couldn't get feature bytes"),
    // };

    // let mut feature_buf: Vec<u8> = vec![];
    // for m in feature_bytes {
    //     feature_buf.push(m.get())
    // }

    // // HARDCODED FOR 1 MODEL
    // // Read the model tensor input types and properly extract ...
    // //
    // let feature: f32 = f32::from_be_bytes([
    //     feature_buf[0],
    //     feature_buf[1],
    //     feature_buf[2],
    //     feature_buf[3],
    // ]);

    // log::info!("Feature Recv: {}", feature);

    // let model = FlatBufferModel::build_from_buffer(model_buf);

    // let model = match model {
    //     Ok(m) => m,
    //     Err(_err) => panic!("cannot init model"),
    // };

    // let resolver = BuiltinOpResolver::default();

    // let builder = InterpreterBuilder::new(model, &resolver).unwrap();
    // let mut interpreter = builder.build().unwrap();

    // interpreter.allocate_tensors().unwrap();

    // let inputs = interpreter.inputs().to_vec();

    // let outputs = interpreter.outputs().to_vec();

    // let input_index = inputs[0];

    // let input_tensor = interpreter.tensor_info(input_index).unwrap();

    // let output_index = outputs[0];
    // let output_tensor = interpreter.tensor_info(output_index).unwrap();
    // log::info!("Model loaded with input tensor: {:?}", input_tensor);
    // log::info!("Model loaded with output tensor: {:?}", output_tensor);

    // let input_tensors: &mut [f32] = interpreter.tensor_data_mut(input_index).unwrap();

    // input_tensors[0] = feature;

    // interpreter.invoke().unwrap();

    // let output: &[f32] = interpreter.tensor_data(output_index).unwrap();

    // log::debug!("Output: {:?}", output);

    return 0;
}

pub fn _debug(ctx: &mut Ctx, ptr:  WasmPtr<u8, Array>, len: u32) -> u32 {

    log::info!("RUNE::DEBUG {}", get_mem_str(ctx, ptr, len));
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
    log::info!("Requesting provider response");

    return 0;
}


#![no_std]
#![feature(alloc, core_intrinsics, lang_items, alloc_error_handler)]
extern crate alloc;
extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
use core::fmt::Write;
use core::panic::PanicInfo;
use core::alloc::Layout;
use alloc::vec::Vec;


use runic_types::*;
use runic_transform::{Transformable};
use normalize::{Transform, Normalize, PipelineContext};


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe{
        let s = match info.payload().downcast_ref::<&str>() {
            Some(s) => s,
            _ => ""
        };

        write!(Wrapper::new(&mut PRINT_BUF), "Panic {}\r\n", s).expect("Can't write");
        debug(&PRINT_BUF); 
        }
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(
    info: Layout, //~ ERROR argument should be `Layout`
) -> ! //~ ERROR return type should be `!`
{
    unsafe {
        write!(Wrapper::new(&mut PRINT_BUF), "{:?} \r\n", info).expect("Can't write");
        debug(&PRINT_BUF); 
    }
    loop {}
}




mod wrapper;
use wrapper::Wrapper;


const PROVIDER_RESPONSE_BUFFER_SIZE: usize = 4096*8;

static mut PROVIDER_RESPONSE_BUFFER: [u8; PROVIDER_RESPONSE_BUFFER_SIZE] =
    [0; PROVIDER_RESPONSE_BUFFER_SIZE];

static mut PRINT_BUF: [u8;512] = [0 as u8; 512];

mod model;

extern "C" {

    fn tfm_model_invoke(
        feature_idx: *const u8,
        feature_len: u32,
    ) -> u32;

    fn tfm_preload_model(
        model_idx: *const u8,
        model_len: u32,
        inputs: u32,
        outputs: u32
    ) -> u32;

    fn _debug(str_ptr: *const u8, str_len: u32) -> u32;

    fn request_capability(ct: u32) -> u32;

    fn request_capability_set_param(idx:u32, key_str_ptr:*const u8, key_str_len:u32, value_ptr:*const u8, value_len:u32, value_type:u32) -> u32;

    fn request_manifest_output(t:u32) -> u32;

    fn request_provider_response(
        provider_response_idx: *const u8,
        max_allowed_provider_response: u32,
        capability_idx: u32
    ) -> u32;

}

fn debug(s: &[u8]) -> u32 {
    unsafe { return _debug(s.as_ptr(), s.len() as u32) }

}


#[no_mangle]
pub extern "C" fn _manifest() -> u32 {
    unsafe {
      tfm_preload_model(model::MODEL.as_ptr(), model::MODEL.len() as u32,  64*3, 64);
 
        
        
        debug(b"Requesting ACCEL Capability");

        let accel_capability_idx = request_capability(CAPABILITY::ACCEL as u32);
        
        
        let key = b"n";       
        let value: &[u8; 4] = &u32::to_be_bytes(64u32); 
        request_capability_set_param(accel_capability_idx, key.as_ptr(), key.len() as u32, value.as_ptr(), value.len() as u32, PARAM_TYPE::INT as u32);

        // 
        request_manifest_output(OUTPUT::SERIAL as u32);
        
    }
    return 1;
}


#[no_mangle]
#[warn(unused_must_use)]
pub extern "C" fn _call(capability_type:i32, input_type:i32, capability_idx:i32) -> i32 {
    
    debug(b"Checking for Data");
    

    unsafe {
        let response_size = request_provider_response(
            PROVIDER_RESPONSE_BUFFER.as_ptr(),
            PROVIDER_RESPONSE_BUFFER_SIZE as u32,
            capability_idx as u32
        );

        debug(b"Trace::request_provider_response done: ");
         
        if response_size > 0 {
            let response_size = response_size as usize;
            if input_type == runic_types::PARAM_TYPE::FLOAT as i32 {
                let accel_sample: &[u8] = &PROVIDER_RESPONSE_BUFFER[0..response_size];
                let accel_sample: Vec<f32> = runic_transform::RTransform::<f32,f32>::from_buffer(&accel_sample.to_vec()).unwrap();
                // debug(b"Trace::request_provider_response returned response");
                let mut input: [f32; 348] = [0.0; 348];
              
                let mut i = 0;
                for element in &mut input {
            
                    *element =  accel_sample[i];
                    i = i+ 1;
                }
              
                debug(b"Finished normalizing\n");
                
                let mut norm_pb: Normalize = Normalize{}; 
                let mut pipeline = PipelineContext{};
                let proc_block_output = norm_pb.transform(input, &mut pipeline);
                let mut proc_block_output_vec: Vec<f32> = Vec::with_capacity(348);
                // for element in &proc_block_output {
            
                //     proc_block_output_vec.push(*element);
                // }
                // let proc_block_output: Vec<u8> = 
                //     runic_transform::RTransform::<f32, f32>::to_buffer(&proc_block_output_vec).unwrap();
            
                debug(b"Finished preparing buffer");
                // Processing 
                
                //  tfm_model_invoke(
                //                 proc_block_output.as_ptr() as *const u8,
                //                 proc_block_output.len() as u32,
                //             );
                
                 return 0 as i32;

            }
            //debug(b"Have a response\r\n");
            // let response_size = response_size as usize;
            // let buf: &[u8] = &PROVIDER_RESPONSE_BUFFER[..response_size ];
            // let proc_block_output = buf;

            //     if input_type == 1 {                    

            //         if capability_type  as i32 == CAPABILITY::RAND  as i32{

                        
            //             tfm_model_invoke(
            //                 proc_block_output.as_ptr() as *const u8,
            //                 proc_block_output.len() as u32,
            //             );
            //             return proc_block_output.len() as i32;

            //         }
            //     }
            // //let provider_response = runic_types::runic_types::ProviderResponse::decode(buf).unwrap();
            // }
            return response_size as i32;
        }

        return 0 as i32;

    }
}

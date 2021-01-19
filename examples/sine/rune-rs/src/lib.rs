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


const PROVIDER_RESPONSE_BUFFER_SIZE: usize = 512;

static mut PROVIDER_RESPONSE_BUFFER: [u8; PROVIDER_RESPONSE_BUFFER_SIZE] =
    [0; PROVIDER_RESPONSE_BUFFER_SIZE];

static mut PRINT_BUF: [u8;512] = [0 as u8; 512];

mod sine_model;

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

//Should be created during runefile-parser


enum CAPABILITY
{
    RAND = 1,
    SOUND = 2,
    ACCEL = 3,
    IMAGE = 4,
    RAW = 5
}

enum PARAM_TYPE {
    INT = 1,
    FLOAT = 2,
    UTF8  = 3,
    BINARY = 4,
}

enum OUTPUT {
    SERIAL = 1,
    BLE = 2,
    PIN = 3,
    WIFI = 4
}


#[no_mangle]
pub extern "C" fn _manifest() -> u32 {
    unsafe {
        tfm_preload_model(sine_model::MODEL.as_ptr(), sine_model::MODEL.len() as u32,  1, 1);
 

        /// SET RAND CAPABILITY
        debug(b"Requesting Rand Capability\r\n");

        let rand_capability_idx = request_capability(CAPABILITY::RAND as u32);
        
        /// SET RAND CAPABILITY PARMS
        let key = b"n";        
        let value: &[u8; 1]= &[1u8]; 
        request_capability_set_param(rand_capability_idx, key.as_ptr(), key.len() as u32, value.as_ptr(), value.len() as u32, PARAM_TYPE::INT as u32);

        //Call output
        request_manifest_output(OUTPUT::SERIAL as u32);
        
    }
    return 1;
}


#[no_mangle]
#[warn(unused_must_use)]
pub extern "C" fn _call(capability_type:i32, input_type:i32, capability_idx:i32) -> i32 {
    
    unsafe {
        let response_size = request_provider_response(
            PROVIDER_RESPONSE_BUFFER.as_ptr(),
            PROVIDER_RESPONSE_BUFFER_SIZE as u32,
            capability_idx as u32
        );

        
        if response_size > 0 {
            //debug(b"Have a response\r\n");
            let response_size = response_size as usize;
            let buf: &[u8] = &PROVIDER_RESPONSE_BUFFER[..response_size ];
            let proc_block_output = buf;

                if input_type == 1 {                    

                    if capability_type  as i32 == CAPABILITY::RAND  as i32{

                        
                        tfm_model_invoke(
                            proc_block_output.as_ptr() as *const u8,
                            proc_block_output.len() as u32,
                        );
                        return proc_block_output.len() as i32;

                    }
                }
            //let provider_response = runic_types::runic_types::ProviderResponse::decode(buf).unwrap();
            }
            return response_size as i32;
    }
}

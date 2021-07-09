use rune_core::{reflect::ReflectionType, Tensor};

#[allow(dead_code)] // these fields are used when we pass them to the runtime
#[repr(C)]
pub(crate) struct TensorRepr {
    descriptor: *const u8,
    descriptor_len: u32,
    data: *mut u8,
}

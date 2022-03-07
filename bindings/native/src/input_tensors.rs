use std::{
    collections::HashMap,
    num::NonZeroUsize,
    os::raw::c_int,
    ptr::{self, NonNull},
    slice,
};

use hotg_rune_runtime::{ElementType, Tensor};

/// A dictionary mapping input node IDs to the tensor that will be passed into
/// the Rune.
///
/// # Safety
///
/// This value must not outlive the `Runtime` it came from. The `Runtime` also
/// shouldn't be used while these `InputTensors` are alive.
pub struct InputTensors(NonNull<HashMap<u32, Tensor>>);

impl std::ops::Deref for InputTensors {
    type Target = HashMap<u32, Tensor>;

    fn deref(&self) -> &Self::Target { unsafe { self.0.as_ref() } }
}

impl std::ops::DerefMut for InputTensors {
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { self.0.as_mut() } }
}

impl From<&'_ mut HashMap<u32, Tensor>> for InputTensors {
    fn from(tensors: &'_ mut HashMap<u32, Tensor>) -> Self {
        InputTensors(tensors.into())
    }
}

#[no_mangle]
pub unsafe extern "C" fn rune_input_tensors_free(tensors: *mut InputTensors) {
    if tensors.is_null() {
        return;
    }

    // Note: NonNull<T> is just a pointer and won't drop the values it points
    // to.
    let _ = Box::from_raw(tensors);
}

#[no_mangle]
pub unsafe extern "C" fn rune_input_tensor_count(
    tensors: Option<NonNull<InputTensors>>,
) -> c_int {
    match tensors {
        Some(t) => t.as_ref().len() as c_int,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rune_input_tensors_get(
    tensors: Option<NonNull<InputTensors>>,
    node_id: u32,
) -> Option<NonNull<Tensor>> {
    tensors
        .map(|mut ptr| ptr.as_mut())
        .and_then(|tensors| tensors.get_mut(&node_id))
        .map(NonNull::from)
}

/// Add a new tensor to this set of input tensors, returning a pointer to the
/// newly created tensor.
///
/// If a tensor has already been declared for this node, it will be overwritten.
///
/// This function may return `null` if the dimensions array contains a zero or
/// `tensors` is a null pointer.
#[no_mangle]
pub unsafe extern "C" fn rune_input_tensors_insert(
    tensors: Option<NonNull<InputTensors>>,
    node_id: u32,
    element_type: ElementType,
    dimensions: *const usize,
    rank: c_int,
) -> *mut Tensor {
    let tensors = match tensors {
        Some(mut t) => t.as_mut(),
        None => return ptr::null_mut(),
    };

    if rank <= 0 {
        return ptr::null_mut();
    }

    let dimensions = slice::from_raw_parts(dimensions, rank as usize);

    let mut dims = Vec::new();

    for &dim in dimensions {
        match usize::try_from(dim).ok().and_then(|d| NonZeroUsize::new(d)) {
            Some(dim) => dims.push(dim),
            None => return ptr::null_mut(),
        }
    }

    tensors.insert(node_id, Tensor::zeroed(element_type, dims));
    tensors
        .get_mut(&node_id)
        .expect("We just inserted this tensor")
}

#[no_mangle]
pub unsafe extern "C" fn rune_tensor_element_type(
    tensor: Option<NonNull<Tensor>>,
) -> ElementType {
    match tensor {
        Some(t) => t.as_ref().element_type(),
        None => ElementType::U8,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rune_tensor_rank(
    tensor: Option<NonNull<Tensor>>,
) -> c_int {
    match tensor {
        Some(t) => t.as_ref().dimensions().len() as c_int,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rune_tensor_dimensions(
    tensor: Option<NonNull<Tensor>>,
) -> *const usize {
    // Note: It's fine to cast *const NonZeroUsize to *const usize.
    match tensor {
        Some(t) => t.as_ref().dimensions().as_ptr().cast(),
        None => ptr::null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn rune_tensor_buffer_len(
    tensor: Option<NonNull<Tensor>>,
) -> c_int {
    tensor
        .map(|t| t.as_ref())
        .map(|t| t.buffer().len())
        .unwrap_or(0) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn rune_tensor_buffer(
    tensor: Option<NonNull<Tensor>>,
) -> *mut u8 {
    tensor
        .map(|mut t| t.as_mut())
        .map(|t| t.buffer_mut().as_mut_ptr())
        .unwrap_or(ptr::null_mut())
}

/// Get a readonly reference to this `Tensor`'s buffer.
#[no_mangle]
pub unsafe extern "C" fn rune_tensor_buffer_readonly(
    tensor: *const Tensor,
) -> *const u8 {
    if tensor.is_null() {
        return ptr::null();
    }

    let tensor = &*tensor;
    tensor.buffer().as_ptr()
}

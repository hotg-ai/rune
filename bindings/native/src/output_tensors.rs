use std::{collections::HashMap, os::raw::c_int, ptr};

use hotg_rune_runtime::{OutputTensor, Tensor};

/// An iterator over each of the tensors returned by the Rune.
///
/// # Safety
///
/// This directly refers to data structures inside the `Runtime`. Any use of
/// the `Runtime` while this reference is alive may invalidate it, causing
/// undefined behaviour.
pub struct OutputTensors<'rt>(
    Box<dyn Iterator<Item = (u32, &'rt OutputTensor)> + 'rt>,
);

impl<'rt> From<&'rt HashMap<u32, Vec<OutputTensor>>> for OutputTensors<'rt> {
    fn from(tensors: &'rt HashMap<u32, Vec<OutputTensor>>) -> Self {
        let iter = tensors
            .iter()
            .flat_map(|(id, tensors)| tensors.iter().map(|t| (*id, t)));

        OutputTensors(Box::new(iter))
    }
}

#[no_mangle]
pub unsafe extern "C" fn rune_output_tensors_free(tensors: *mut OutputTensors) {
    if tensors.is_null() {
        return;
    }

    let _ = Box::from_raw(tensors);
}

/// Ask the `OutputTensors` iterator for the next `OutputTensor` and the ID of
/// the node it came from.
///
/// This will return `false` if you have reached the end of the iterator.
#[no_mangle]
pub unsafe extern "C" fn rune_output_tensors_next(
    tensors: *mut OutputTensors<'_>,
    id_out: *mut u32,
    tensor_out: *mut *const OutputTensor,
) -> bool {
    if tensors.is_null() {
        return false;
    }

    match (&mut *tensors).0.next() {
        Some((id, tensor)) => {
            id_out.write(id);
            tensor_out.write(tensor);
            true
        },
        None => false,
    }
}

/// Get a reference to the underlying `Tensor` if this output tensor has a fixed
/// size.
///
/// This will return `null` if the `OutputTensor` contains dynamically sized
/// data (i.e. strings) or if the `tensor` parameter is `null`.
///
/// # Safety
///
/// This inherits all the safety requirements from `OutputTensors`, with the
/// added condition that you must not mutate the tensor's data through this
/// pointer.
#[no_mangle]
unsafe extern "C" fn rune_output_tensor_as_fixed(
    tensor: *const OutputTensor,
) -> *const Tensor {
    if tensor.is_null() {
        return ptr::null();
    }

    match &*tensor {
        OutputTensor::Tensor(t) => t,
        OutputTensor::StringTensor { .. } => ptr::null(),
    }
}

#[no_mangle]
unsafe extern "C" fn rune_output_tensor_as_string_tensor<'tensor>(
    tensor: *const OutputTensor,
) -> *const StringTensor<'tensor> {
    if tensor.is_null() {
        return ptr::null();
    }

    match &*tensor {
        OutputTensor::Tensor(_) => ptr::null_mut(),
        OutputTensor::StringTensor {
            dimensions,
            strings,
        } => Box::into_raw(Box::new(StringTensor {
            dimensions,
            strings,
        })),
    }
}

/// A wrapper around a tensor containing dynamically sized elements (i.e.
/// strings).
///
/// # Note
///
/// Users must free this object once they are done with it.
///
/// # Safety
///
/// This inherits all the safety requirements from `OutputTensors`, with the
/// added condition that you must not mutate the tensor's data through this
/// pointer.
pub struct StringTensor<'tensor> {
    dimensions: &'tensor [usize],
    strings: &'tensor [String],
}

/// Get the number of dimensions in this `StringTensor`.
#[no_mangle]
pub unsafe extern "C" fn rune_string_tensor_rank(
    tensor: *const StringTensor<'_>,
) -> usize {
    if tensor.is_null() {
        return 0;
    }

    (&*tensor).dimensions.len()
}

/// Get a pointer to this `StringTensor`'s dimensions.
#[no_mangle]
pub unsafe extern "C" fn rune_string_tensor_dimensions(
    tensor: *const StringTensor<'_>,
) -> *const usize {
    if tensor.is_null() {
        return ptr::null();
    }

    (&*tensor).dimensions.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn rune_string_tensor_free(
    tensor: *const StringTensor<'_>,
) {
    if tensor.is_null() {
        return;
    }

    let _ = Box::from_raw(tensor as *mut StringTensor);
}

/// Get a pointer to the string at a specific index in the `StringTensor`'s
/// backing array, returning its length in bytes and setting `string_out` if
/// that string exists.
///
/// If the index is out of bounds, this function returns `0` and sets
/// `string_out` to `null`.
#[no_mangle]
pub unsafe extern "C" fn rune_string_tensor_get_by_index(
    tensor: *const StringTensor<'_>,
    index: usize,
    string_out: *mut *const u8,
) -> c_int {
    if tensor.is_null() {
        string_out.write(ptr::null());
        return 0;
    }

    match (&*tensor).strings.get(index) {
        Some(s) => {
            string_out.write(s.as_ptr());
            s.len() as c_int
        },
        None => {
            string_out.write(ptr::null());
            0
        },
    }
}

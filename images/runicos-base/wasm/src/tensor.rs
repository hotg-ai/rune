use rune_core::{reflect::ReflectionType, Tensor};

const _ASSERT_USIZE_CAN_TRANSMUTE_TO_U32: fn() = || unsafe {
    let _: u32 = core::mem::transmute(0_usize);
};

#[allow(dead_code)] // these fields are used when we pass them to the runtime
#[repr(C)]
pub(crate) struct TensorRepr {
    element_type: u32,
    num_dimensions: u32,
    dimensions: *const u32,
    data: *mut u8,
}

impl TensorRepr {
    pub unsafe fn new(
        element_type: u32,
        num_dimensions: u32,
        dimensions: *const u32,
        data: *mut u8,
    ) -> Self {
        TensorRepr {
            element_type,
            num_dimensions,
            dimensions,
            data,
        }
    }

    /// # Safety
    ///
    /// This holds raw pointers to the underlying [`Tensor`]. It is up to the
    /// caller to ensure lifetimes hold and that mutation is only done through
    /// these pointers when the original reference was mutable.
    pub unsafe fn from_ref<T>(tensor: &Tensor<T>) -> Self
    where
        T: ReflectionType,
    {
        let element_type = T::TYPE.runtime_id().unwrap();
        let dimensions = tensor.dimensions();
        let num_dimensions = dimensions.len() as u32;
        let data = tensor.elements().as_ptr() as *const u8 as *mut u8;

        TensorRepr::new(
            element_type,
            num_dimensions,
            dimensions.as_ptr() as *const u32,
            data,
        )
    }

    /// # Safety
    ///
    /// This holds raw pointers to the underlying [`Tensor`]. It is up to the
    /// caller to ensure lifetimes hold and that mutation is only done through
    /// these pointers when the original reference was mutable.
    pub unsafe fn from_mut<T>(tensor: &mut Tensor<T>) -> Self
    where
        T: ReflectionType + Clone,
    {
        let element_type = T::TYPE.runtime_id().unwrap();
        let num_dimensions = tensor.rank() as u32;

        // Safety: we need to make sure this tensor isn't shared
        let data = tensor.make_elements_mut().as_mut_ptr() as *mut u8;

        assert_eq!(
            core::mem::size_of::<u32>(),
            core::mem::size_of::<usize>(),
            "You can't transmute a [usize] to [u32]"
        );
        // Safety: because we implemented `Tensor`, we can guarantee that `data`
        // and `dimensions` are actually separate borrows.
        let dimensions = tensor.dimensions().as_ptr() as *const u32;

        TensorRepr::new(element_type, num_dimensions, dimensions, data)
    }
}

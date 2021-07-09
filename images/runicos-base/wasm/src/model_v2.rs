use rune_core::{Shape, TensorList, TensorListMut};
use alloc::{
    vec::Vec,
    string::{String, ToString},
};
use core::marker::PhantomData;
use crate::intrinsics::StringRef;

const TFLITE_MIMETYPE: &str = "application/tflite-model";

#[derive(Debug, Clone, PartialEq)]
pub struct Model<Input, Output> {
    id: u32,
    input_shapes: Vec<Shape<'static>>,
    output_shapes: Vec<Shape<'static>>,
    _types: PhantomData<fn(Input) -> Output>,
}

impl<Input, Output> Model<Input, Output> {
    pub fn new(
        model_data: &[u8],
        input_shapes: &[Shape<'static>],
        output_shapes: &[Shape<'static>],
    ) -> Self {
        let id = unsafe {
            let input_shape_descriptors: Vec<String> =
                input_shapes.iter().map(|s| s.to_string()).collect();
            let input_shape_descriptors: Vec<_> = input_shape_descriptors
                .iter()
                .map(|s| StringRef::from(s.as_str()))
                .collect();
            let output_shape_descriptors: Vec<String> =
                output_shapes.iter().map(|s| s.to_string()).collect();
            let output_shape_descriptors: Vec<_> = output_shape_descriptors
                .iter()
                .map(|s| StringRef::from(s.as_str()))
                .collect();

            crate::intrinsics::rune_model_load(
                TFLITE_MIMETYPE.as_ptr(),
                TFLITE_MIMETYPE.len() as u32,
                model_data.as_ptr(),
                model_data.len() as u32,
                input_shape_descriptors.as_ptr(),
                input_shape_descriptors.len() as u32,
                output_shape_descriptors.as_ptr(),
                output_shape_descriptors.len() as u32,
            )
        };

        Model {
            id,
            input_shapes: input_shapes.into(),
            output_shapes: output_shapes.into(),
            _types: PhantomData,
        }
    }
}

impl<Input, Output> Model<Input, Output>
where
    for<'a> &'a Input: TensorList<'a>,
    Output: TensorListMut,
{
    pub fn process(&mut self, inputs: Input) -> Output {
        assert_eq!(
            (&inputs).shape_list().as_ref(),
            &self.input_shapes,
            "The input had the wrong shape",
        );
        let mut outputs = <Output>::new_tensors(&self.output_shapes);

        unsafe {
            let inputs = (&inputs).element_ptr();
            let mut outputs = <Output>::element_ptr_mut(&mut outputs);

            crate::intrinsics::rune_model_infer(
                self.id,
                inputs.as_ref().as_ptr(),
                outputs.as_mut().as_mut_ptr(),
            );
        }

        outputs
    }
}

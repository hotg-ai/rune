#![no_std]
use hotg_runecoral::{
    mimetype, AccelerationBackend, ElementType, Error, InferenceContext,
    LoadError, Tensor, TensorDescriptor, TensorMut,
};

#[test]
fn test_this_model() {
    let model = include_bytes!("model.tflite").to_vec();

    let mut ctx =
        InferenceContext::create_context(mimetype(), &model, AccelerationBackend::NONE).unwrap();

    let input = [0; 96 * 96];
    let mut output = [0; 3];

    ctx.infer(
        &[Tensor::from_slice(&input, &[96, 96])],
        &mut [TensorMut::from_slice(&mut output, &[1, 1, 1, 3])],
    )
    .unwrap();

    println!("Output {:?}", output);
    // assert_eq!(output, [17; 130; 10]);
}

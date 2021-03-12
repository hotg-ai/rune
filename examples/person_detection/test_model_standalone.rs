#![no_std]
use tflite::ops::builtin::BuiltinOpResolver;
use tflite::{FlatBufferModel, InterpreterBuilder};

fn get_resource(res: &str) -> PathBuf {
        
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
      d.push(res);
    return d;
}

fn one_byte(mut buf: &mut [u8], byte: u8) {
    buf.write(&[byte]); 
}


fn get_model_output() {
    
    let model = FlatBufferModel::build_from_file(get_resource("model.tflite").as_os_str()).unwrap();

    let resolver = BuiltinOpResolver::default();

    let builder = InterpreterBuilder::new(model, &resolver).unwrap();
    let mut interpreter = builder.build().unwrap();

    interpreter.allocate_tensors().unwrap();

    let inputs = interpreter.inputs().to_vec();
    let outputs = interpreter.outputs().to_vec();

    let input_index = inputs[0];
    let output_index =  outputs[0];
    let input_data: [[u8; 96]; 96] =[[0; 96]; 96];
    
    let mut input_tensors: &mut [u8] = interpreter.tensor_data_mut(input_index).unwrap();

    input_tensors.write(input_data).unwrap();
    
    interpreter.invoke().unwrap();

    let output: &[u8] = interpreter.tensor_data(output_index).unwrap();
    output

}


#[test]
fn test_this_model() {

    let output = get_model_output();
    println!("printing: {}", output)
    // assert_eq!(output, [17; 130; 10]);

}    
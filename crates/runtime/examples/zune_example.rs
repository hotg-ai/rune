use hotg_rune_runtime::zune::{ZuneEngine, TensorResult, ElementType};

fn main() {
    println!("Hello World!");

    let sine_zune = include_bytes!("sine.rune");
    let mut zune_engine = ZuneEngine::load(sine_zune).expect("Unable to initialize Zune Engine!");

    println!("input nodes {:?}",  zune_engine.input_nodes());
    println!("output nodes {:?}",  zune_engine.output_nodes());
    println!("input tensor names of rand => {:?}", zune_engine.get_input_tensor_names("rand"));
    println!("output tensor names of sine => {:?}", zune_engine.get_output_tensor_names("sine"));

    let input_tensor = TensorResult {
        element_type: ElementType::F32,
        dimensions: vec![1,1],
        buffer: vec![ 0, 0, 0, 0]
    };

    zune_engine.set_input_tensor("rand", "input", &input_tensor);

    println!("input tensor rand => {:?}", zune_engine.get_input_tensor("rand", "input"));

    zune_engine.predict().expect("Failed to run predict!");

    println!("input tensor for serial: => {:?}", zune_engine.get_input_tensor("serial", "input"));
}
use anyhow::{Context, Error};
use hotg_rune_runtime::zune::{ElementType, TensorResult, ZuneEngine};

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    let filename = args.get(1).map(|s| s.as_str()).unwrap_or("/home/helios/Code/hotg/rune/crates/runtime/examples/sine.rune");

    let sine_zune = std::fs::read(&filename)
        .with_context(|| format!("Unable to read \"{filename}\""))?;

    let mut zune_engine = ZuneEngine::load(&sine_zune)
        .context("Unable to initialize Zune Engine!")?;

    println!("input nodes {:?}", zune_engine.input_nodes());
    println!("output nodes {:?}", zune_engine.output_nodes());
    println!(
        "input tensor names of rand => {:?}",
        zune_engine.get_input_tensor_names("rand")
    );
    println!(
        "input tensor names of sine => {:?}",
        zune_engine.get_input_tensor_names("sine")
    );
    println!(
        "output tensor names of sine => {:?}",
        zune_engine.get_output_tensor_names("sine")
    );

    let input_tensor = TensorResult {
        element_type: ElementType::F32,
        dimensions: vec![1, 1],
        buffer: vec![0, 0, 0, 0],
    };

    zune_engine.set_input_tensor("rand", "input", &input_tensor);

    println!(
        "input tensor rand => {:?}",
        zune_engine.get_input_tensor("rand", "input")
    );

    zune_engine.predict().context("Failed to run predict!")?;

    println!(
        "output tensor for sine: => {:?}",
        zune_engine.get_output_tensor("sine", "Identity")
    );

    for node in zune_engine.output_nodes() {
        let input_tensor_names = zune_engine.get_input_tensor_names(node)?;
        for tensor_name in &input_tensor_names {
            println!("Output {:?} {:?}: {:?}", node, tensor_name, zune_engine.get_input_tensor(node, tensor_name));
        }
    }

    Ok(())
}

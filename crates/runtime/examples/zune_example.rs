use anyhow::{Context, Error};
use hotg_rune_runtime::zune::{ElementType, Tensor, ZuneEngine};

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    let default_example_path = format!("{}/../../examples/zune/sine.zune",env!("CARGO_MANIFEST_DIR"));
    let filename = args.get(1).map(|s| s.as_str()).unwrap_or(&default_example_path);

    let sine_zune = std::fs::read(&filename)
        .with_context(|| format!("Unable to read \"{filename}\""))?;

    let mut zune_engine = ZuneEngine::load(&sine_zune)
        .context("Unable to initialize Zune Engine!")?;

    println!("input tensors {:?}", zune_engine.input_tensor_names());
    println!("output tensors {:?}", zune_engine.output_tensor_names());
    println!(
        "input tensor constraint => {:?}",
        zune_engine.get_input_tensor_constraint("mod360", "input")
    );

    let input_tensor = Tensor {
        element_type: ElementType::F32,
        dimensions: vec![1, 1],
        buffer: 0.0_f32.to_ne_bytes().to_vec(),
    };

    zune_engine.set_input_tensor("mod360", "input", &input_tensor)?;

    println!(
        "input tensor for mod360 => {:?}",
        zune_engine.get_input_tensor("mod360", "input")
    );

    zune_engine.run().context("Failed to run!")?;

    println!(
        "output tensor for sine: => {:?}",
        zune_engine.get_output_tensor("sine", "Identity")
    );

    Ok(())
}

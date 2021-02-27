use rune_runtime::{DefaultEnvironment, Runtime};

const SINE: &[u8] = include_bytes!("../../examples/sine/sine.rune");

#[test]
fn load_the_sine_rune() {
    let _ = env_logger::try_init();
    let env = DefaultEnvironment::with_seed([0; 32]);

    let _runtime = Runtime::load(SINE, env).unwrap();
}

#[test]
fn run_the_sine_rune() {
    let _ = env_logger::try_init();
    let env = DefaultEnvironment::with_seed([0; 32]);

    let mut runtime = Runtime::load(SINE, env).unwrap();

    runtime.call().unwrap();
}

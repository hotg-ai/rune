use rune_runtime::{DefaultEnvironment, Runtime};

const SINE: &[u8] = include_bytes!("../../examples/sine/sine.rune");

#[test]
fn load_the_sine_rune() {
    let env = DefaultEnvironment::with_seed([0; 32]);

    let _runtime = Runtime::load(SINE, env).unwrap();
}

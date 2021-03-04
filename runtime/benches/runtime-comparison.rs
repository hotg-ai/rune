use criterion::{criterion_group, criterion_main, Criterion};
use runic_types::Transform;
use rand::Rng;
use tflite::{
    FlatBufferModel, Interpreter, InterpreterBuilder,
    ops::builtin::BuiltinOpResolver,
};
use std::path::Path;
use modulo::Modulo;
use rune_syntax::{Diagnostics};
use rune_codegen::Compilation;
use rune_runtime::{DefaultEnvironment, Environment, Runtime};

fn compile_sine(optimized: bool) -> Vec<u8> {
    let src = include_str!("../../examples/sine/Runefile");
    let parsed = rune_syntax::parse(src).unwrap();
    let mut diags = Diagnostics::new();
    let rune = rune_syntax::analyse(0, &parsed, &mut diags);
    assert!(!diags.has_errors());

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let sine_dir = repo_root.join("examples").join("sine");

    let working_dir = tempfile::tempdir().unwrap();

    let compilation = Compilation {
        name: String::from("sine"),
        rune,
        rune_project_dir: repo_root.to_path_buf(),
        current_directory: sine_dir,
        working_directory: working_dir.path().to_path_buf(),
        optimized,
    };

    rune_codegen::generate(compilation).unwrap()
}

pub fn execute_sine(c: &mut Criterion) {
    // Note: We reuse the same runtime across different benchmark calls here.
    // Ideally we'd create a new runtime every time, but that's incredibly
    // expensive.

    let mut group = c.benchmark_group("execute sine");

    let wasm = compile_sine(true);
    let mut runtime =
        Runtime::load(&wasm, DefaultEnvironment::default()).unwrap();
    group.bench_function("optimised rune", |b| {
        b.iter(|| runtime.call().unwrap())
    });

    let wasm = compile_sine(false);
    let mut runtime =
        Runtime::load(&wasm, DefaultEnvironment::default()).unwrap();
    group.bench_function("debug rune", |b| b.iter(|| runtime.call().unwrap()));

    let mut manual = ManualSine::load();
    group.bench_function("no rune", |b| b.iter(|| manual.call()));
}

struct ManualSine {
    env: DefaultEnvironment,
    modulo: Modulo<f32>,
    interpreter: Interpreter<'static, BuiltinOpResolver>,
}

impl ManualSine {
    fn load() -> Self {
        let env = DefaultEnvironment::default();
        let modulo = Modulo::default().with_modulus(360.0);

        let model = include_bytes!("../../examples/sine/sinemodel.tflite");

        let model = FlatBufferModel::build_from_buffer(model.to_vec()).unwrap();
        let resolver = BuiltinOpResolver::default();
        let builder = InterpreterBuilder::new(model, resolver).unwrap();
        let mut interpreter = builder.build().unwrap();
        interpreter.allocate_tensors().unwrap();

        ManualSine {
            env,
            modulo,
            interpreter,
        }
    }

    fn call(&mut self) -> f32 {
        const SIZE_OF_FLOAT: usize = std::mem::size_of::<f32>();

        let random_data: f32 = self.env.rng().unwrap().gen();
        let within_360: f32 = self.modulo.transform(random_data);

        let input_ix = self.interpreter.inputs()[0];
        let buffer = self.interpreter.tensor_buffer_mut(input_ix).unwrap();
        let raw = within_360.to_le_bytes();
        buffer[..SIZE_OF_FLOAT].copy_from_slice(&raw);

        let output_ix = self.interpreter.outputs()[0];
        let buffer = self.interpreter.tensor_buffer(output_ix).unwrap();
        let mut raw_float = [0; SIZE_OF_FLOAT];
        raw_float.copy_from_slice(&buffer[..SIZE_OF_FLOAT]);

        f32::from_le_bytes(raw_float)
    }
}

criterion_group!(benches, execute_sine);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, Criterion};
use runic_types::Transform;
use tflite::{
    FlatBufferModel, Interpreter, InterpreterBuilder,
    ops::builtin::BuiltinOpResolver,
};
use std::path::Path;
use modulo::Modulo;
use rune_syntax::{Diagnostics};
use rune_codegen::Compilation;
use rune_runtime::{DefaultEnvironment, Environment, Runtime};

fn main() {
    env_logger::init();

    criterion_main!(benches);
    main();
}

criterion_group!(
    benches,
    execute_sine,
    // compile_times,
    runtime_startup
);

pub fn compile_times(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile");

    group
        .bench_function("debug-sine", |b| b.iter(|| compile_sine(false)))
        .bench_function("release-sine", |b| b.iter(|| compile_sine(true)))
        .bench_function("debug-gesture", |b| b.iter(|| compile_gesture(false)))
        .bench_function("release-gesture", |b| {
            b.iter(|| compile_gesture(true))
        });
}

pub fn runtime_startup(c: &mut Criterion) {
    let mut group = c.benchmark_group("startup");

    let sine = compile_sine(true);
    let gesture = compile_gesture(true);
    let env = DefaultEnvironment::default();

    group
        .bench_function("sine", |b| {
            b.iter(|| Runtime::load(&sine, env.clone()).unwrap())
        })
        .bench_function("gesture", |b| {
            b.iter(|| Runtime::load(&gesture, env.clone()).unwrap())
        });
}

pub fn execute_sine(c: &mut Criterion) {
    // Note: We reuse the same runtime across different benchmark calls here.
    // Ideally we'd create a new runtime every time, but that's incredibly
    // expensive.

    let mut group = c.benchmark_group("execute-sine");

    let wasm = compile_sine(true);
    let mut runtime =
        Runtime::load(&wasm, DefaultEnvironment::default()).unwrap();
    group.bench_function("optimised-rune", |b| {
        b.iter(|| runtime.call().unwrap())
    });

    let wasm = compile_sine(false);
    let mut runtime =
        Runtime::load(&wasm, DefaultEnvironment::default()).unwrap();
    group.bench_function("debug-rune", |b| b.iter(|| runtime.call().unwrap()));

    let mut manual = ManualSine::load();
    group.bench_function("no-rune", |b| b.iter(|| manual.call()));
}

pub fn execute_gesture(c: &mut Criterion) {
    let mut group = c.benchmark_group("execute-gesture");

    let wasm = compile_gesture(true);
    let mut runtime =
        Runtime::load(&wasm, DefaultEnvironment::default()).unwrap();
    group.bench_function("optimised-rune", |b| {
        b.iter(|| runtime.call().unwrap())
    });

    let wasm = compile_gesture(false);
    let mut runtime =
        Runtime::load(&wasm, DefaultEnvironment::default()).unwrap();
    group.bench_function("debug-rune", |b| b.iter(|| runtime.call().unwrap()));
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

        let mut random_data = [0; SIZE_OF_FLOAT];
        self.env.fill_random(&mut random_data).unwrap();
        let within_360: f32 =
            self.modulo.transform(f32::from_le_bytes(random_data));

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

fn compile_sine(optimized: bool) -> Vec<u8> {
    let src = include_str!("../../examples/sine/Runefile");
    compile("sine", src, optimized)
}

fn compile_gesture(optimized: bool) -> Vec<u8> {
    let src = include_str!("../../examples/gesture/Runefile");
    compile("gesture", src, optimized)
}

fn compile(name: &str, runefile: &str, optimized: bool) -> Vec<u8> {
    let parsed = rune_syntax::parse(runefile).unwrap();
    let mut diags = Diagnostics::new();
    let rune = rune_syntax::analyse(0, &parsed, &mut diags);
    assert!(!diags.has_errors());

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let sine_dir = repo_root.join("examples").join(name);

    let working_dir = tempfile::tempdir().unwrap();

    let compilation = Compilation {
        name: String::from(name),
        rune,
        rune_project_dir: repo_root.to_path_buf(),
        current_directory: sine_dir,
        working_directory: working_dir.path().to_path_buf(),
        optimized,
    };

    rune_codegen::generate(compilation).unwrap()
}

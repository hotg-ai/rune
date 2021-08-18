use criterion::{criterion_group, criterion_main, Criterion};
use tempdir::TempDir;
use std::path::{Path, PathBuf};
use anyhow::Context;
use hotg_rune_cli::run::{
    Image,
    image::ImageSource,
    Sound,
    sound::AudioClip,
    new_capability_switcher,
};

use hotg_rune_wasmer_runtime::Runtime;
use hotg_runicos_base_runtime::BaseImage;
use hotg_rune_core::capabilities;

use hotg_rune_codegen::{
    Compilation, DefaultEnvironment, RuneProject, Verbosity,
};

use hotg_rune_syntax::{hir::Rune, yaml::Document, Diagnostics};

pub fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|path| path.join(".git").exists())
        .expect("Unable to determine the project's root directory. Where is \".git/\"?")
        .to_path_buf()
}

pub fn example_dir() -> PathBuf { project_root().join("examples") }

fn load_rune(path: PathBuf) -> Vec<u8> {
    std::fs::read(&path).with_context(|| {
        format!("Unable to read \"{}\"", path.display())
    }).unwrap()
}

fn parse_runefile(runefile: &Path) -> Rune {
    let src = std::fs::read_to_string(runefile).unwrap();
    let mut diags = Diagnostics::new();
    let parsed = Document::parse(&src).unwrap();
    let rune = hotg_rune_syntax::analyse_yaml_runefile(&parsed, &mut diags);
    assert!(!diags.has_errors());
    rune
}

fn build_rune(rune_path: PathBuf, name: String, rune: Rune) {
    let rune_build_dir = TempDir::new("rune_build_dir").unwrap();

    let compilation = Compilation {
        name,
        rune,
        working_directory: rune_build_dir.path().to_path_buf(),
        current_directory: rune_path,
        rune_project: RuneProject::Disk(project_root()),
        optimized: true,
        verbosity: Verbosity::Quiet,
    };

    let mut env = DefaultEnvironment::for_compilation(&compilation);

    let blob = hotg_rune_codegen::generate_with_env(compilation, &mut env)
        .expect("Rune compilation failed");

    assert_ne!(blob.len(), 0);
}

fn sine_build_benchmark(c: &mut Criterion) {
    let base = example_dir().join("sine");
    let runefile = base.join("Runefile.yml");
    let rune = parse_runefile(&runefile);

    c.bench_function("sine_build",
        |b| b.iter(|| { build_rune(base.clone(), String::from("sine"), rune.clone()) }));
}

fn sine_inference_benchmark(c: &mut Criterion) {
    let mut runtime = Runtime::load(&load_rune(example_dir().join("sine").join("sine.rune")),
                                    BaseImage::with_defaults())
        .context("Unable to create rune runtime")
        .unwrap();

    c.bench_function("sine_inference",
        |b| b.iter(|| { runtime.call().context("Call Failed").unwrap() }));
}

fn microspeech_inference_benchmark(c: &mut Criterion) {
    let base = example_dir().join("microspeech");

    let mut img = BaseImage::with_defaults();
    let wav_file = base.join("data").join("right").join("fb7eb481_nohash_0.wav");
    img.register_capability(
        capabilities::SOUND,
        new_capability_switcher::<Sound, _>(vec![AudioClip::from_wav_file(wav_file).unwrap()]));

    let mut runtime = Runtime::load(&load_rune(base.join("microspeech.rune")), img)
        .context("Unable to create rune runtime")
        .unwrap();

    c.bench_function("microspeech_inference",
        |b| b.iter(|| runtime.call().context("Call Failed").unwrap()));
}

fn styletransfer_inference_benchmark(c: &mut Criterion) {
    let base = example_dir().join("style_transfer");

    let mut img = BaseImage::with_defaults();
    img.register_capability(
        capabilities::IMAGE,
        new_capability_switcher::<Image, _>(vec![ImageSource::from_file(base.join("style.jpg")).unwrap(),
                                         ImageSource::from_file(base.join("flower.jpg")).unwrap()]));

    let mut runtime = Runtime::load(&load_rune(base.join("style_transfer.rune")), img)
        .context("Unable to create rune runtime")
        .unwrap();

    c.bench_function("styletransfer_inference",
        |b| b.iter(|| runtime.call().context("Call Failed").unwrap()));
}

criterion_group!(
    name = build_benchmark;
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = sine_build_benchmark);

criterion_group!(
    name = inference_benchmark;
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = sine_inference_benchmark, microspeech_inference_benchmark, styletransfer_inference_benchmark);

criterion_main!(build_benchmark, inference_benchmark);

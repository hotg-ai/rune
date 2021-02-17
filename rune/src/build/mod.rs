use anyhow::{Context, Error};
use cargo::{
    core::{
        compiler::{BuildConfig, CompileMode},
        shell::Verbosity,
        Shell, Workspace,
    },
    ops::{CompileFilter, CompileOptions, Packages},
    Config,
};
use std::fs;

pub fn build(fileloc: &str) -> Result<(), Error> {
    let contents = fs::read_to_string(fileloc)
        .with_context(|| format!("Failed to load \"{}\"", fileloc))?;

    let homedir = runefile_parser::parser::generate(contents);

    let config = Config::default().context("Couldn't make workspace config")?;
    config.shell().set_verbosity(Verbosity::Quiet);

    let manifest_path = homedir.join("Cargo.toml");
    let rune_file =
        homedir.join("target/wasm32-unknown-unknown/release/rune.wasm");

    let ws = Workspace::new(&manifest_path, &config)
        .context("Couldn't make workspace")?;

    let mut compile_opts = CompileOptions::new(
        &Config::new(Shell::default(), homedir.clone(), homedir),
        CompileMode::Build,
    )
    .context("Couldn't create the compile options")?;

    compile_opts.build_config = BuildConfig::new(
        &config,
        None,
        &[String::from("wasm32-unknown-unknown")],
        CompileMode::Build,
    )
    .context("Unable to create the build config")?;

    compile_opts.spec = Packages::Default;
    compile_opts.filter = CompileFilter::Default {
        required_features_filterable: true,
    };

    // 
    //  * MEGA HUNT DOWN IN THE CARGO CODE ... this is what I had to do
    //
    //    cd /Users/kthakore/Documents/HOTG-ai/cargo; cargo build --quiet; cd
    // /Users/kthakore/.rune/runes/b39f1b98-dbe4-4c2d-98b6-06f544647d4b; echo
    // "BUILD CARGO"; cargo clean;
    // /Users/kthakore/Documents/HOTG-ai/cargo/target/release/cargo build
    // --target wasm32-unknown-unknown --release > DEV_LOG; cd
    // /Users/kthakore/Documents/HOTG-ai/cargo;    cd /Users/kthakore/
    // Documents/HOTG-ai/cargo; cargo build --release --quiet; cd
    // /Users/kthakore/.rune/runes/b39f1b98-dbe4-4c2d-98b6-06f544647d4b; echo
    // "BUILD CARGO"; cargo clean;
    // /Users/kthakore/Documents/HOTG-ai/cargo/target/release/cargo build
    // --target wasm32-unknown-unknown  > DEV_LOG; cd
    // /Users/kthakore/Documents/HOTG-ai/cargo;

    //    Then I DIFFED that to find out that --release tag was going to
    // profiles!!! No DOCS ANYWHERE!

    compile_opts.build_config.requested_profile = "release".into();

    cargo::ops::compile(&ws, &compile_opts).context("Couldn't compile Rune")?;
    log::info!("Rune compiled");

    log::info!("Create Rune: {:?}", rune_file);

    Ok(())
}

use log;
use std::fs;
use clap::ArgMatches;

pub fn build(fileloc: &str) {
    let contents = fs::read_to_string(fileloc);

    let contents = match contents {
        Ok(c) => c,
        Err(_err) => {
            log::error!("Failed to load file '{}'", fileloc);
            return;
        }
    };

    let homedir = runefile_parser::parser::generate(contents);

    let config = match cargo::Config::default() {
        Ok(c) => c,
        Err(err) => {
            log::error!("Couldn't make workspace config {:?}", err);
            return;
        }
    };
    let mut manifest_path = homedir.clone(); 
    manifest_path.push("Cargo.toml");

    let ws = match cargo::core::Workspace::new(&manifest_path, &config) {
        Ok(w) => w,
        Err(err) => {
            log::error!("Couldn't make workspace {:?}", err);
            return;
        }
    };

    let compile_opts = match cargo::ops::CompileOptions::new(
        &cargo::Config::new(cargo::core::Shell::default(), homedir.clone(), homedir),
        cargo::core::compiler::CompileMode::Build // we should also do a test before
    )  {
        Ok(co) => co,
        Err(err) => {
            log::error!("Couldn't compile Rune '{:?}'", err);
            return;
        }
    };

    println!("{:?}", compile_opts);
    match cargo::ops::compile(&ws, &compile_opts) {
        Ok(_x) => log::info!("Rune compiled"),
        Err(err) => {
            log::error!("Couldn't compile Rune '{:?}'", err);
            return;
        }
    }
    
}

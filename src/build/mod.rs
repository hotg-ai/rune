use log;
use std::fs;

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

    let mut config = match cargo::Config::default() {
        Ok(c) => c,
        Err(err) => {
            log::error!("Couldn't make workspace config {:?}", err);
            return;
        }
    };
    config.shell().set_verbosity( cargo::core::shell::Verbosity::Quiet);
    

    let mut manifest_path = homedir.clone(); 
    manifest_path.push("Cargo.toml");

    let mut rune_file = homedir.clone();
    rune_file.push("target/wasm32-unknown-unknown/release/rune.wasm");

    let ws = match cargo::core::Workspace::new(&manifest_path, &config) {
        Ok(w) => w,
        Err(err) => {
            log::error!("Couldn't make workspace {:?}", err);
            return;
        }
    };

    

    let mut compile_opts = match cargo::ops::CompileOptions::new(
        &cargo::Config::new(cargo::core::Shell::default(), homedir.clone(), homedir),
        cargo::core::compiler::CompileMode::Build // we should also do a test before
    )  {
        Ok(co) => co,
        Err(err) => {
            log::error!("Couldn't compile Rune '{:?}'", err);
            return;
        }
    };

    compile_opts.build_config = cargo::core::compiler::BuildConfig::new(
            &config, 
            None, 
            &[String::from("wasm32-unknown-unknown")],
            cargo::core::compiler::CompileMode::Build
        ).unwrap();

    compile_opts.spec = cargo::ops::Packages::Default; 
    compile_opts.filter = cargo::ops::CompileFilter::Default{
        required_features_filterable: true,
    };
    
    
    //  * MEGA HUNT DOWN IN THE CARGO CODE ... this is what I had to do
    //  
    //    cd /Users/kthakore/Documents/HOTG-ai/cargo; cargo build --quiet; cd /Users/kthakore/.rune/runes/b39f1b98-dbe4-4c2d-98b6-06f544647d4b; echo "BUILD CARGO"; cargo clean; /Users/kthakore/Documents/HOTG-ai/cargo/target/release/cargo build --target wasm32-unknown-unknown --release > DEV_LOG; cd /Users/kthakore/Documents/HOTG-ai/cargo;
    //    cd /Users/kthakore/Documents/HOTG-ai/cargo; cargo build --release --quiet; cd /Users/kthakore/.rune/runes/b39f1b98-dbe4-4c2d-98b6-06f544647d4b; echo "BUILD CARGO"; cargo clean; /Users/kthakore/Documents/HOTG-ai/cargo/target/release/cargo build --target wasm32-unknown-unknown  > DEV_LOG; cd /Users/kthakore/Documents/HOTG-ai/cargo;

    //    Then I DIFFED that to find out that --release tag was going to profiles!!! No DOCS ANYWHERE!

    compile_opts.build_config.requested_profile = cargo::util::interning::InternedString::new("release");
   
    match cargo::ops::compile(&ws, &compile_opts) {
        Ok(_x) => log::info!("Rune compiled"),
        Err(err) => {
            log::error!("Couldn't compile Rune '{:?}'", err);
            return;
        }
    }

    log::info!("Create Rune:{:?}", rune_file);

    // 
    
}

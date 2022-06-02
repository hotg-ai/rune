use std::path::PathBuf;

use hotg_rune_compiler::{
    codegen::{Codegen, CodegenStorage},
    asset_loader::{AssetLoader, DefaultAssetLoader, ReadError},
    im::Vector,
    parse::{Frontend, FrontendStorage},
    BuildConfig, Environment, EnvironmentStorage, FeatureFlags,
};
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
use uriparse::URI;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::default()
                .add_directive("hotg_rune_compiler=debug".parse().unwrap())
                .add_directive("compile=debug".parse().unwrap()),
        )
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let runefile = std::env::args()
        .nth(1)
        .expect("Usage: validate-runefile-schema <runefile>");
    let runefile = PathBuf::from(runefile);

    let src = std::fs::read_to_string(&runefile).unwrap();
    let parent = runefile.parent().unwrap().to_path_buf();

    let mut db = Database::default();
    db.set_src(src.into());
    db.set_config(BuildConfig {
        current_directory: parent.clone(),
        features: FeatureFlags::stable(),
    });

    let archive = db.rune_archive().unwrap();

    let name = parent.file_name().unwrap();
    std::fs::write(parent.join(name).with_extension("zip"), &*archive).unwrap();
}

#[derive(Default)]
#[salsa::database(FrontendStorage, EnvironmentStorage, CodegenStorage)]
struct Database {
    storage: salsa::Storage<Self>,
    fs: DefaultAssetLoader,
}

impl salsa::Database for Database {}

// The parsing process requires you to load proc-blocks and read files. You
// can satisfy these dependencies by implementing the corresponding traits.

impl AssetLoader for Database {
    fn read(&self, uri: &URI<'_>) -> Result<Vector<u8>, ReadError> {
        self.fs.read(uri)
    }
}

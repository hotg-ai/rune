mod cargo_build;
mod components;
mod write_project_to_disk;

use std::sync::Arc;

pub use self::components::*;
use crate::{inputs::Inputs, BuildContext, codegen::Codegen};

#[salsa::query_group(CompileGroup)]
pub trait Compile: Inputs + Codegen {
    #[salsa::dependencies]
    fn build(&self) -> Result<CompiledBinary, Arc<CompileError>>;
}

fn build(db: &dyn Compile) -> Result<CompiledBinary, Arc<CompileError>> {
    let ctx = db.build_context();
    let files = db.files();

    for file in &files {
        write_project_to_disk::run(file, &ctx);
    }

    let BuildContext {
        name,
        working_directory,
        optimized,
        verbosity,
        ..
    } = &*ctx;

    cargo_build::build(name, working_directory, *optimized, *verbosity)
        .map_err(Arc::new)
}

mod cargo_build;
mod components;
mod write_project_to_disk;

pub use self::components::*;

use crate::Phase;

pub fn phase() -> Phase {
    Phase::new()
        .and_then(write_project_to_disk::run_system)
        .and_then(cargo_build::run_system)
}

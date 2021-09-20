//! The type checking phase.

use crate::phases::Phase;

mod load_resource_data;

pub fn phase() -> Phase {
    Phase::new().and_then(load_resource_data::run_system)
}

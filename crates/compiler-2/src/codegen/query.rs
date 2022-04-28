use std::sync::Arc;

use im::Vector;

use crate::parse::{DocumentV1, Frontend};

#[salsa::query_group(CodegenStorage)]
pub trait Codegen: Frontend {
    /// Create a self-contained [`DocumentV1`] with all resources resolved and
    ///
    fn self_contained_runefile(&self) -> Arc<DocumentV1>;

    fn runez(&self) -> Vector<u8>;
}

#[tracing::instrument(skip(db))]
fn self_contained_runefile(db: &dyn Codegen) -> Arc<DocumentV1> {
    todo!();
}

#[tracing::instrument(skip(db))]
fn runez(db: &dyn Codegen) -> Vector<u8> {
    todo!();
}

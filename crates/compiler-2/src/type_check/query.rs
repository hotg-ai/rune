use crate::lowering::HirDB;

/// Queries related to constructing the Rune's data pipeline.
#[salsa::query_group(TypeCheckGroup)]
pub trait TypeCheck: HirDB {}

use crate::lowering::HirDB;

/// Queries related to constructing the Rune's data pipeline.
#[salsa::query_group(TypeCheckGroup)]
pub trait TypeCheck: HirDB {
    /// asdf
    #[salsa::interned]
    fn edge(&self, edge: Edge) -> EdgeId;
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Edge;

intern_id! {
    pub struct EdgeId(salsa::InternId);
}

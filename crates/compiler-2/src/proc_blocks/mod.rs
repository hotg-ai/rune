use std::sync::Arc;

use im::Vector;

use crate::{type_check::TensorShape, Text};

/// Something which knows how to load proc-blocks.
pub trait ProcBlockRegistry: Send + Sync + 'static {
    /// Load a proc-block and call its `graph()` function to find out what its
    /// inputs and outputs are.
    fn load_graph(
        &self,
        path: crate::parse::Path,
    ) -> Result<Tensors, Arc<dyn std::error::Error>>;
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct Tensors {
    pub inputs: Vector<(Text, TensorShape)>,
    pub outputs: Vector<(Text, TensorShape)>,
}

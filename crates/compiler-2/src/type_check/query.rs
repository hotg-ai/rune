use im::Vector;

use crate::lowering::{HirDB, NodeId, NodeKind};

/// Queries related to constructing the Rune's data pipeline.
#[salsa::query_group(TypeCheckGroup)]
pub trait TypeCheck: HirDB {
    fn output_nodes(&self) -> Vector<NodeId>;
}

fn output_nodes(db: &dyn TypeCheck) -> Vector<NodeId> {
    let (names, _) = db.names();

    names
        .values()
        .copied()
        .filter_map(|id| id.as_node())
        .filter_map(|id| {
            let node = db.lookup_node(id);
            match node.kind {
                NodeKind::Output => Some(id),
                _ => None,
            }
        })
        .collect()
}

use legion::systems::CommandBuffer;

use crate::{
    hir::{Name, PipelineNode},
    yaml::DocumentV1,
};

/// Goes through and registers all the named items and their locations in the
/// Runefile.
#[legion::system]
pub(crate) fn run(cmd: &mut CommandBuffer, #[resource] doc: &DocumentV1) {
    for (name, stage) in &doc.pipeline {
        cmd.push((Name::from(name), stage.span(), PipelineNode));
    }

    for (name, decl) in &doc.resources {
        let name = Name::from(name);
        cmd.push((name, decl.span()));
    }
}

use legion::systems::CommandBuffer;
use crate::{BuildContext, codegen::CustomSection};

/// Embed a [`crate::codegen::RuneVersion`] in the Rune as a [`CustomSection`].
#[legion::system]
pub(crate) fn run(cmd: &mut CommandBuffer, #[resource] ctx: &BuildContext) {
    if let Some(components) = version_section(ctx) {
        cmd.push((components,));
    }
}

fn version_section(ctx: &BuildContext) -> Option<CustomSection> {
    ctx.rune_version.as_ref().map(|version| {
        version
            .as_custom_section()
            .expect("We should always be able to serialize to JSON")
    })
}

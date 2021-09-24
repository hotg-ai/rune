use legion::{Entity, Query, systems::CommandBuffer, world::SubWorld};
use std::convert::TryFrom;
use crate::{
    BuildContext,
    codegen::{CustomSection, RESOURCE_CUSTOM_SECTION, RuneGraph},
    lowering::{Name, ResourceData},
};

/// Generate the various [`CustomSection`]s that we want to embed in the
/// generated Rune.
///
/// There are roughly three custom sections:
///
/// - [`RuneGraph`]
/// - [`RuneVersion`]
/// - [`ResourceData`] - one instance for each resource
#[legion::system]
pub(crate) fn run(
    cmd: &mut CommandBuffer,
    world: &SubWorld,
    #[resource] ctx: &BuildContext,
    resources: &mut Query<(Entity, &Name, &ResourceData)>,
) {
    if let Some(components) = version_section(ctx) {
        cmd.push((components,));
    }

    resources.for_each(world, |(&entity, name, data)| {
        cmd.add_component(entity, inline_resource(name, data));
    });

    let graph = rune_graph();
    let graph_section = graph
        .as_custom_section()
        .expect("We should always be able to serialize to JSON");
    cmd.push((graph, graph_section));
}

fn rune_graph() -> RuneGraph { RuneGraph {} }

fn inline_resource(name: &Name, data: &ResourceData) -> CustomSection {
    let name_len = u32::try_from(name.len()).unwrap();
    let data_len = u32::try_from(data.len()).unwrap();
    let buffer_length = std::mem::size_of_val(&name_len)
        + name.len()
        + std::mem::size_of_val(&data_len)
        + data.len();
    let mut buffer = Vec::with_capacity(buffer_length);

    buffer.extend(name_len.to_be_bytes());
    buffer.extend_from_slice(name.as_bytes());
    buffer.extend(data_len.to_be_bytes());
    buffer.extend_from_slice(data);

    CustomSection {
        section_name: RESOURCE_CUSTOM_SECTION.to_string(),
        value: buffer.into(),
    }
}

fn version_section(ctx: &BuildContext) -> Option<CustomSection> {
    ctx.rune_version.as_ref().map(|version| {
        version
            .as_custom_section()
            .expect("We should always be able to serialize to JSON")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_custom_section_for_a_resource() {
        let data = ResourceData::from(&b"Hello, World!"[..]);
        let name = Name::from("my_resource");

        let CustomSection {
            section_name,
            value,
        } = inline_resource(&name, &data);

        assert_eq!(section_name, RESOURCE_CUSTOM_SECTION);
        let (resource_name, resource_data, rest) =
            hotg_rune_core::inline_resource_from_bytes(&value).unwrap();
        assert_eq!(resource_name, name.as_str());
        assert_eq!(resource_data, data.as_ref());
        assert!(rest.is_empty());
    }
}

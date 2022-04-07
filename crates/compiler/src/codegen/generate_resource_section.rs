use std::convert::TryFrom;

use legion::{systems::CommandBuffer, world::SubWorld, Entity, Query};

use crate::{
    codegen::{CustomSection, RESOURCE_CUSTOM_SECTION},
    lowering::{Name, ResourceData},
};

/// Generate [`CustomSection`]s that embed each resource's default value in
/// the Rune.
#[legion::system]
pub(crate) fn run(
    cmd: &mut CommandBuffer,
    world: &SubWorld,
    resources: &mut Query<(Entity, &Name, &ResourceData)>,
) {
    resources.for_each(world, |(&entity, name, data)| {
        cmd.add_component(entity, inline_resource(name, data));
    });
}

pub(crate) fn inline_resource(
    name: &Name,
    data: &ResourceData,
) -> CustomSection {
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
            hotg_rune_core::decode_inline_resource(&value).unwrap();
        assert_eq!(resource_name, name.as_str());
        assert_eq!(resource_data, data.as_ref());
        assert!(rest.is_empty());
    }
}

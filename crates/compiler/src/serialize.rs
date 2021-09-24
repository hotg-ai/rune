use legion::{
    Registry, World,
    serialize::{Canon, DeserializeNewWorld},
    storage::Component,
};
use serde::{Deserializer, Serialize, Serializer, de::DeserializeSeed};

pub(crate) trait RegistryExt {
    fn register_with_type_name<C>(&mut self) -> &mut Self
    where
        C: Component + serde::Serialize + for<'de> serde::Deserialize<'de>;
}

impl RegistryExt for Registry<String> {
    fn register_with_type_name<C>(&mut self) -> &mut Self
    where
        C: Component + serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        self.register::<C>(std::any::type_name::<C>().to_string());
        self
    }
}

/// Create a new [`Registry`] populated with the various component types used
/// in this crate.
pub fn registry() -> Registry<String> {
    let mut registry = Registry::new();

    crate::parse::register_components(&mut registry);
    crate::lowering::register_components(&mut registry);
    crate::type_check::register_components(&mut registry);
    crate::codegen::register_components(&mut registry);

    registry
}

pub fn serialize_world<S>(
    world: &World,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let registry = registry();
    let canon = Canon::default();

    world
        .as_serializable(legion::any(), &registry, &canon)
        .serialize(serializer)
}

pub fn deserialize_world<'de, D>(deserializer: D) -> Result<World, D::Error>
where
    D: Deserializer<'de>,
{
    let registry = crate::serialize::registry();
    let canon = Canon::default();

    DeserializeNewWorld {
        world_deserializer: &registry,
        entity_serializer: &canon,
    }
    .deserialize(deserializer)
}

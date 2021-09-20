use std::{path::PathBuf, sync::Arc};
use legion::{
    World,
    serialize::{Canon, DeserializeNewWorld},
};
use serde::{Deserialize, Serialize, de::DeserializeSeed};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct File {
    pub path: PathBuf,
    pub data: Arc<[u8]>,
}

impl File {
    pub fn new(path: impl Into<PathBuf>, data: impl Into<Arc<[u8]>>) -> Self {
        File {
            path: path.into(),
            data: data.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CustomSection {
    pub name: String,
    pub value: Vec<u8>,
}

#[derive(Debug)]
pub struct Rune(pub World);

impl Serialize for Rune {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let registry = crate::serialize::registry();
        let canon = Canon::default();

        self.0
            .as_serializable(legion::any(), &registry, &canon)
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Rune {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let registry = crate::serialize::registry();
        let canon = Canon::default();

        let world = DeserializeNewWorld {
            world_deserializer: &registry,
            entity_serializer: &canon,
        }
        .deserialize(deserializer)?;

        Ok(Rune(world))
    }
}

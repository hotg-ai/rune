use std::{ops::Deref, sync::Arc};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResourceData(pub Arc<[u8]>);

impl<T: Into<Arc<[u8]>>> From<T> for ResourceData {
    fn from(data: T) -> Self { ResourceData(data.into()) }
}

impl Deref for ResourceData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModelData(pub Arc<[u8]>);

impl<A: Into<Arc<[u8]>>> From<A> for ModelData {
    fn from(data: A) -> Self { ModelData(data.into()) }
}

impl Deref for ModelData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target { &self.0 }
}

use std::{path::Path, sync::Arc};

use bytes::Bytes;

use crate::{BuildContext, FeatureFlags};

pub trait FileSystem {
    fn read_file(&self, path: &Path) -> Result<Bytes, std::io::Error> {
        std::fs::read(path).map(Bytes::from)
    }
}

#[salsa::query_group(InputsGroup)]
pub trait Inputs: FileSystem {
    #[salsa::input]
    fn build_context(&self) -> Arc<BuildContext>;
    #[salsa::input]
    fn feature_flags(&self) -> FeatureFlags;
}

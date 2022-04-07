use std::{path::Path, sync::Arc};

use im::Vector;

use crate::{BuildContext, FeatureFlags};

pub trait FileSystem {
    fn read_file(&self, path: &Path) -> Result<Vector<u8>, std::io::Error> {
        std::fs::read(path).map(Vector::from)
    }
}

#[salsa::query_group(InputsGroup)]
pub trait Inputs: FileSystem {
    #[salsa::input]
    fn build_context(&self) -> Arc<BuildContext>;
    #[salsa::input]
    fn feature_flags(&self) -> FeatureFlags;
}

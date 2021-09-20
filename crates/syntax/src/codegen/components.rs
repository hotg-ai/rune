use std::{path::PathBuf, sync::Arc};

#[derive(Debug, Clone, PartialEq)]
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

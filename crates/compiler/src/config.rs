use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BuildConfig {
    pub current_directory: PathBuf,
}

/// The build environment.
#[salsa::query_group(EnvironmentStorage)]
pub trait Environment {
    #[salsa::input]
    fn config(&self) -> BuildConfig;
}

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BuildConfig {
    /// The directory all paths are resolved relative to.
    pub current_directory: PathBuf,
    /// Unstable features which can enable extra options.
    pub features: FeatureFlags,
}

/// Flags used by the Rune compiler to enable experimental features.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct FeatureFlags {}

/// The build environment.
#[salsa::query_group(EnvironmentStorage)]
pub trait Environment {
    #[salsa::input]
    fn config(&self) -> BuildConfig;
}

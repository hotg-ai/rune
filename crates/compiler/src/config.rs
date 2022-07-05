use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BuildConfig {
    /// The directory all paths are resolved relative to.
    pub current_directory: PathBuf,
    /// Unstable features which can enable extra options.
    pub features: FeatureFlags,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            current_directory: std::env::current_dir()
                .expect("Unable to determine the current directory"),
            features: FeatureFlags::stable(),
        }
    }
}

/// Flags used by the Rune compiler to enable features.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct FeatureFlags {}

impl FeatureFlags {
    /// Enable all stable features.
    pub fn stable() -> Self {
        FeatureFlags {}
    }
}

/// The build environment.
#[salsa::query_group(EnvironmentStorage)]
pub trait Environment {
    #[salsa::input]
    fn config(&self) -> BuildConfig;
}

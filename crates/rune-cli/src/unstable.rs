use std::path::PathBuf;
use hotg_rune_compiler::FeatureFlags;

/// The various unstable/internal flags you can use with Rune.
///
/// # Note to Implementors
///
/// To make sure people explicitly opt into unstable features and have a
/// consistent experience, there are a couple requirements:
///
/// - Set `global = true` so unstable flags can be placed anywhere on the
///   command-line
/// - Set `requires = "enable_unstable"` so you can only use unstable features
///   after explicitly opting in.
#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Unstable {
    /// Enable unstable features.
    #[structopt(long, global = true)]
    pub unstable: bool,
    /// (unstable) A path to the Rune repository. Primarily used to patch
    /// dependencies when hacking on Rune locally.
    #[structopt(
        long,
        requires = "unstable",
        parse(from_os_str),
        global = true
    )]
    rune_repo_dir: Option<PathBuf>,
}

impl Unstable {
    pub fn feature_flags(&self) -> FeatureFlags {
        let mut features = FeatureFlags::default();

        if !self.unstable {
            return features;
        }

        features.set_rune_repo_dir(self.rune_repo_dir.clone());

        features
    }
}

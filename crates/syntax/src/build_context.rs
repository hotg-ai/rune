use std::{path::PathBuf, process::Command};

/// Inputs used during the compilation process.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuildContext {
    /// The name of the Rune being compiled.
    pub name: String,
    /// The `Runefile.yml` source text.
    pub runefile: String,
    /// A directory that can be used for any temporary artifacts.
    pub working_directory: PathBuf,
    /// The directory that all paths (e.g. to models) are resolved relative to.
    pub current_directory: PathBuf,
    /// Generate an optimized build.
    pub optimized: bool,
    pub verbosity: Verbosity,
}

impl BuildContext {
    /// Create a new [`BuildContext`] using the convention that the
    /// [`BuildContext.name`] is named after the
    /// [`BuildContext.current_directory`].
    pub fn for_directory(
        directory: impl Into<PathBuf>,
    ) -> Result<BuildContext, std::io::Error> {
        let current_directory = directory.into();
        let working_directory = current_directory.clone();

        let name = current_directory
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unable to determine the current directory's name",
                )
            })?;

        let runefile = current_directory.join("Runefile.yml");
        let runefile = std::fs::read_to_string(runefile)?;

        Ok(BuildContext {
            name,
            runefile,
            working_directory,
            current_directory,
            optimized: true,
            verbosity: Verbosity::Normal,
        })
    }

    #[cfg(test)]
    pub(crate) fn from_doc(doc: crate::yaml::Document) -> Self {
        BuildContext {
            name: "rune".to_string(),
            runefile: serde_yaml::to_string(&doc).unwrap(),
            working_directory: PathBuf::from("."),
            current_directory: PathBuf::from("."),
            optimized: false,
            verbosity: Verbosity::Normal,
        }
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}

impl Verbosity {
    pub fn from_quiet_and_verbose(quiet: bool, verbose: bool) -> Option<Self> {
        match (verbose, quiet) {
            (true, false) => Some(Verbosity::Verbose),
            (false, true) => Some(Verbosity::Quiet),
            (false, false) => Some(Verbosity::Normal),
            (true, true) => None,
        }
    }

    /// Add a `--quiet` or `--verbose` argument to the command if necessary.
    pub fn add_flags(&self, cmd: &mut Command) {
        match self {
            Verbosity::Quiet => {
                cmd.arg("--quiet");
            },
            Verbosity::Verbose => {
                cmd.arg("--verbose");
            },
            Verbosity::Normal => {},
        }
    }
}

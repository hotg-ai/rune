mod abi_v0;
mod abi_v1;

use std::{path::PathBuf, str::FromStr};

use anyhow::{Context, Error};
use codespan_reporting::term::termcolor::ColorChoice;
use once_cell::sync::Lazy;

use crate::Unstable;

#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Build {
    /// The Runefile to compile.
    #[structopt(parse(from_os_str), default_value = "Runefile.yml")]
    runefile: PathBuf,
    /// Where to write the generated Rune.
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
    /// The directory to use when caching builds.
    #[structopt(long, env)]
    cache_dir: Option<PathBuf>,
    /// The directory that all paths are resolved relative to (Defaults to the
    /// Runefile's directory)
    #[structopt(short, long, env)]
    current_dir: Option<PathBuf>,
    /// The name of the Rune (defaults to the Runefile directory's name).
    #[structopt(short, long)]
    name: Option<String>,
    /// Hide output from tools that rune may call.
    #[structopt(short, long, conflicts_with = "verbose")]
    quiet: bool,
    /// Prints even more detailed information.
    #[structopt(short, long, conflicts_with = "quiet")]
    verbose: bool,
    /// Compile the Rune without optimisations.
    #[structopt(long)]
    debug: bool,
    /// The type of Rune to build.
    #[structopt( long, short, env = "RUNE_TARGET", default_value = "abi-v0", parse(try_from_str), possible_values = &["abi-v0", "abi-v1"])]
    target: Target,
}

impl Build {
    pub fn execute(
        self,
        color: ColorChoice,
        unstable: Unstable,
    ) -> Result<(), Error> {
        match self.target {
            Target::AbiV0 => abi_v0::execute(self, color, unstable),
            Target::AbiV1 => abi_v1::execute(self, unstable),
        }
    }

    pub fn current_directory(&self) -> Result<PathBuf, Error> {
        if let Some(dir) = &self.current_dir {
            return Ok(dir.clone());
        }

        if let Some(parent) =
            self.runefile.parent().and_then(|p| p.canonicalize().ok())
        {
            return Ok(parent);
        }

        std::env::current_dir()
            .context("Unable to determine the current directory")
    }

    pub fn name(&self) -> Result<String, Error> {
        if let Some(name) = &self.name {
            return Ok(name.clone());
        }

        let current_dir = self.current_directory()?;

        if let Some(name) = current_dir.file_name().and_then(|n| n.to_str()) {
            return Ok(name.to_string());
        }

        Err(Error::msg("Unable to determine the Rune's name"))
    }
}

pub(crate) static DEFAULT_CACHE_DIR: Lazy<String> = Lazy::new(|| {
    let cache_dir = dirs::cache_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| PathBuf::from("."));

    cache_dir
        .join("rune")
        .join("runes")
        .to_string_lossy()
        .into_owned()
});

#[derive(Debug, Clone, PartialEq)]
enum Target {
    AbiV0,
    AbiV1,
}

impl FromStr for Target {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "abi-v0" => Ok(Target::AbiV0),
            "abi-v1" => Ok(Target::AbiV1),
            _ => Err(Error::msg("Unknown ABI version")),
        }
    }
}

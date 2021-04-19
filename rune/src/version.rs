use anyhow::Error;
use build_info::{BuildInfo, GitInfo};
use structopt::StructOpt;
use std::{path::Path, str::FromStr};

build_info::build_info!(pub fn version);

#[derive(Debug, Clone, PartialEq, StructOpt)]
pub struct Version {
    #[structopt(short, long)]
    pub verbose: bool,
    #[structopt(short, long, default_value = "text")]
    pub format: Format,
}

impl Version {
    pub fn execute(self) -> Result<(), Error> {
        let binary = std::env::args().next().expect("");
        let version = version();
        let git = version.version_control.as_ref().unwrap().git().unwrap();

        match self.format {
            Format::Text => print_text(&binary, version, git, self.verbose),
            Format::Json => todo!(),
        }

        Ok(())
    }
}

fn print_text(binary: &str, version: &BuildInfo, git: &GitInfo, verbose: bool) {
    let executable = Path::new(binary)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or(binary);

    // We want to copy rustc
    // rustc 1.53.0-nightly (5a4ab2645 2021-04-18)
    println!(
        "{} {} ({} {})",
        executable,
        version.crate_info.version,
        git.commit_short_id,
        version.timestamp.format("%Y-%m-%d"),
    );

    if !verbose {
        return;
    }

    println!("binary: {}", executable);
    println!("rune-version: {}", version.crate_info.version);
    println!("commit-hash: {}", git.commit_id);
    println!("commit-date: {}", git.commit_timestamp.format("%Y-%m-%d"));
    println!("host: {}", version.compiler.target_triple);
    println!("rustc-version: {}", version.compiler.version);
    if let Some(commit_hash) = version.compiler.commit_id.as_ref() {
        println!("rustc-commit-hash: {}", commit_hash);
    }
    if let Some(commit_date) = version.compiler.commit_date {
        println!("rustc-commit-date: {}", commit_date.format("%Y-%m-%d"));
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Format {
    Text,
    Json,
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(Format::Text),
            "json" => Ok(Format::Json),
            _ => Err(Error::msg("Expected \"text\" or \"json\"")),
        }
    }
}

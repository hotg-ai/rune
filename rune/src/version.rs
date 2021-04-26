use anyhow::Error;
use build_info::{
    chrono::{DateTime, NaiveDate, Utc},
};
use structopt::StructOpt;
use std::{borrow::Cow, io::Write, path::Path, str::FromStr};

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
        let binary = std::env::args_os().next().expect("");
        let executable = Path::new(&binary).file_name().unwrap_or(&binary);

        let version = version();
        let git = version.version_control.as_ref().unwrap().git().unwrap();

        let info = VersionInfo {
            executable: executable.to_string_lossy(),
            rune_version: version.crate_info.version.to_string(),
            commit_hash: &git.commit_id,
            commit_short_hash: &git.commit_short_id,
            commit_timestamp: git.commit_timestamp,
            host: &version.compiler.target_triple,
            rustc_version: version.compiler.version.to_string(),
            rustc_commit_hash: version.compiler.commit_id.as_deref(),
            rustc_commit_date: version.compiler.commit_date,
        };

        match self.format {
            Format::Text => print_text(&info, self.verbose),
            Format::Json => print_json(&info, self.verbose)?,
        }

        Ok(())
    }
}

fn print_json(info: &VersionInfo, verbose: bool) -> Result<(), Error> {
    let mut stdout = std::io::stdout();

    if verbose {
        serde_json::to_writer_pretty(&mut stdout, info)?;
    } else {
        serde_json::to_writer(&mut stdout, info)?;
    }

    writeln!(stdout)?;
    stdout.flush()?;

    Ok(())
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
struct VersionInfo<'a> {
    executable: Cow<'a, str>,
    rune_version: String,
    commit_short_hash: &'a str,
    commit_hash: &'a str,
    commit_timestamp: DateTime<Utc>,
    host: &'a str,
    rustc_version: String,
    rustc_commit_hash: Option<&'a str>,
    rustc_commit_date: Option<NaiveDate>,
}

fn print_text(info: &VersionInfo<'_>, verbose: bool) {
    let VersionInfo {
        executable,
        rune_version,
        commit_short_hash,
        commit_hash,
        commit_timestamp,
        host,
        rustc_version,
        rustc_commit_hash,
        rustc_commit_date,
    } = info;

    // We want to copy rustc
    // rustc 1.53.0-nightly (5a4ab2645 2021-04-18)
    println!(
        "{} {} ({} {})",
        executable,
        rune_version,
        commit_short_hash,
        commit_timestamp.format("%Y-%m-%d"),
    );

    if !verbose {
        return;
    }

    println!("binary: {}", executable);
    println!("rune-version: {}", rune_version);
    println!("commit-hash: {}", commit_hash);
    println!("commit-date: {}", commit_timestamp.to_rfc3339());
    println!("host: {}", host);
    println!("rustc-version: {}", rustc_version);
    if let Some(commit_hash) = rustc_commit_hash {
        println!("rustc-commit-hash: {}", commit_hash);
    }
    if let Some(commit_date) = rustc_commit_date {
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

use std::{borrow::Cow, io::Write, path::Path};

use anyhow::Error;
use build_info::chrono::{DateTime, NaiveDate, Utc};
use structopt::StructOpt;
use strum::VariantNames;

use crate::Format;

build_info::build_info!(pub fn version);

#[derive(Debug, Clone, PartialEq, StructOpt)]
pub struct Version {
    #[structopt(short, long)]
    pub verbose: bool,
    #[structopt(short, long, default_value = "text", possible_values = Format::VARIANTS)]
    pub format: Format,
}

impl Version {
    pub fn execute(self) -> Result<(), Error> {
        let binary = std::env::args_os().next().expect("");
        let executable = Path::new(&binary).file_name().unwrap_or(&binary);

        let version = version();
        let git = version.version_control.as_ref().and_then(|v| v.git());

        let info = VersionInfo {
            executable: executable.to_string_lossy(),
            rune_version: version.crate_info.version.to_string(),
            commit_hash: git.map(|g| g.commit_id.as_str()),
            commit_short_hash: git.map(|g| g.commit_short_id.as_str()),
            commit_timestamp: git.map(|g| g.commit_timestamp),
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
    commit_short_hash: Option<&'a str>,
    commit_hash: Option<&'a str>,
    commit_timestamp: Option<DateTime<Utc>>,
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

    // We want to copy rustc but need to take into account times when git info
    // doesn't exist...
    // rustc 1.53.0-nightly (5a4ab2645 2021-04-18)
    match (
        commit_short_hash,
        commit_timestamp.map(|ts| ts.format("%Y-%m-%d")),
    ) {
        (Some(h), Some(ts)) => {
            println!("{} {} ({} {})", executable, rune_version, h, ts,)
        },
        (Some(h), None) => println!("{} {} ({})", executable, rune_version, h),
        (None, Some(ts)) => {
            println!("{} {} ({})", executable, rune_version, ts)
        },
        (None, None) => println!("{} {}", executable, rune_version),
    }

    if !verbose {
        return;
    }

    println!("binary: {}", executable);
    println!("rune-version: {}", rune_version);
    if let Some(commit_hash) = commit_hash {
        println!("commit-hash: {}", commit_hash);
    }
    if let Some(commit_timestamp) = commit_timestamp {
        println!("commit-date: {}", commit_timestamp.to_rfc3339());
    }
    println!("host: {}", host);
    println!("rustc-version: {}", rustc_version);
    if let Some(commit_hash) = rustc_commit_hash {
        println!("rustc-commit-hash: {}", commit_hash);
    }
    if let Some(commit_date) = rustc_commit_date {
        println!("rustc-commit-date: {}", commit_date.format("%Y-%m-%d"));
    }
}

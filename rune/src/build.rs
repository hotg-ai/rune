use anyhow::{Context, Error};
use codespan_reporting::{
    files::SimpleFiles,
    term::{termcolor::StandardStream, Config, termcolor::ColorChoice},
};
use rune_codegen::Compilation;
use rune_syntax::Diagnostics;
use std::{
    env::current_dir,
    path::{PathBuf},
};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Build {
    /// The Runefile to compile.
    #[structopt(parse(from_os_str))]
    runefile: PathBuf,
    /// Where to write the generated Rune.
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
    /// The directory to use when caching builds.
    #[structopt(long, env, default_value = &**DEFAULT_CACHE_DIR)]
    cache_dir: PathBuf,
    /// The directory that all paths are resolved relative to (Defaults to the
    /// Runefile's directory)
    #[structopt(short, long, env)]
    current_dir: Option<PathBuf>,
    /// The name of the Rune (defaults to the Runefile directory's name).
    #[structopt(short, long)]
    name: Option<String>,
    /// Compile the Rune without optimisations.
    #[structopt(long)]
    debug: bool,
}

impl Build {
    pub fn execute(self, color: ColorChoice) -> Result<(), Error> {
        let src =
            std::fs::read_to_string(&self.runefile).with_context(|| {
                format!("Unable to read \"{}\"", self.runefile.display())
            })?;

        let mut files = SimpleFiles::new();
        let id = files.add(self.runefile.display().to_string(), &src);

        log::debug!("Parsing \"{}\"", self.runefile.display());
        let parsed = rune_syntax::parse(&src).unwrap();

        let mut diags = Diagnostics::new();
        let rune = rune_syntax::analyse(id, &parsed, &mut diags);

        let mut writer = StandardStream::stdout(color);
        let config = Config::default();

        for diag in &diags {
            codespan_reporting::term::emit(&mut writer, &config, &files, diag)
                .context("Unable to print the diagnostic")?;
        }

        if diags.has_errors() {
            anyhow::bail!("Aborting compilation due to errors.");
        }

        let current_directory = self.current_directory()?;
        let name = self.name()?;

        let working_directory = self.cache_dir.join(&name);
        let dest = self.output.unwrap_or_else(|| {
            current_directory.join(&name).with_extension("rune")
        });

        log::debug!(
            "Compiling {} in \"{}\"",
            name,
            working_directory.display()
        );
        let compilation = Compilation {
            name,
            rune,
            rune_project_dir: nearest_git_repo(),
            current_directory,
            working_directory,
            optimized: !self.debug,
        };
        let blob = rune_codegen::generate(compilation)
            .context("Rune compilation failed")?;

        log::debug!("Generated {} bytes", blob.len());

        std::fs::write(&dest, &blob).with_context(|| {
            format!("Unable to write to \"{}\"", dest.display())
        })?;

        Ok(())
    }

    fn current_directory(&self) -> Result<PathBuf, Error> {
        if let Some(dir) = &self.current_dir {
            return Ok(dir.clone());
        }

        if let Some(parent) = self.runefile.parent() {
            return Ok(parent.to_path_buf());
        }

        current_dir()
            .and_then(|p| p.canonicalize())
            .context("Unable to determine the current directory")
    }

    fn name(&self) -> Result<String, Error> {
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

fn nearest_git_repo() -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();

    for parent in current_dir.ancestors() {
        if parent.join(".git").exists() {
            return parent.to_path_buf();
        }
    }

    panic!("Unable to find the rune project root");
}

static DEFAULT_CACHE_DIR: Lazy<String> = Lazy::new(|| {
    let cache_dir = dirs::cache_dir()
        .or_else(|| dirs::home_dir())
        .unwrap_or_else(|| PathBuf::from("."));

    cache_dir.join("runes").to_string_lossy().into_owned()
});

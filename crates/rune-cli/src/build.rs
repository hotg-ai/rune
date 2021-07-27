use anyhow::{Context, Error};
use codespan_reporting::{
    files::SimpleFile,
    term::{termcolor::StandardStream, Config, termcolor::ColorChoice},
};
use hotg_rune_codegen::{
    Compilation, DefaultEnvironment, GitSpecifier, RuneProject, Verbosity,
};
use hotg_rune_syntax::{hir::Rune, yaml::Document, Diagnostics};
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq, structopt::StructOpt)]
pub struct Build {
    /// The Runefile to compile.
    #[structopt(parse(from_os_str), default_value = "Runefile")]
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
}

impl Build {
    pub fn execute(self, color: ColorChoice) -> Result<(), Error> {
        let verbosity =
            Verbosity::from_quiet_and_verbose(self.quiet, self.verbose)
                .context(
                    "The --verbose and --quiet flags can't be used together",
                )?;

        let rune = analyze(&self.runefile, color)?;

        let current_directory = self.current_directory()?;
        let name = self.name()?;

        let working_directory = self
            .cache_dir
            .unwrap_or_else(|| Path::new(&*DEFAULT_CACHE_DIR).join(&name));
        let dest = self.output.unwrap_or_else(|| {
            current_directory.join(&name).with_extension("rune")
        });

        log::debug!(
            "Compiling {} in \"{}\"",
            name,
            working_directory.display()
        );
        let rune_project = match rune_repo_root() {
            Some(root_dir) => RuneProject::Disk(root_dir),
            None => {
                // looks like we aren't into a checked out rune dir
                let build_info = crate::version::version();
                let git = build_info
                    .version_control
                    .as_ref()
                    .and_then(|v| v.git())
                    .context("Unable to determine the rune project dir")?;
                RuneProject::Git {
                    repo: RuneProject::GITHUB_REPO.into(),
                    specifier: GitSpecifier::Commit(git.commit_id.clone()),
                }
            },
        };
        let compilation = Compilation {
            name,
            rune,
            rune_project,
            current_directory,
            working_directory,
            verbosity,
            optimized: !self.debug,
        };
        let mut env = DefaultEnvironment::for_compilation(&compilation)
            .with_build_info(crate::version::version().clone());
        let blob = hotg_rune_codegen::generate_with_env(compilation, &mut env)
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

        if let Some(parent) =
            self.runefile.parent().and_then(|p| p.canonicalize().ok())
        {
            return Ok(parent);
        }

        std::env::current_dir()
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

fn rune_repo_root() -> Option<PathBuf> {
    let current_dir = std::env::current_dir().unwrap();

    for parent in current_dir.ancestors() {
        if parent.join(".git").exists()
            && parent.join("images").exists()
            && parent.join("proc-blocks").exists()
        {
            return Some(parent.to_path_buf());
        }
    }

    None
}

static DEFAULT_CACHE_DIR: Lazy<String> = Lazy::new(|| {
    let cache_dir = dirs::cache_dir()
        .or_else(|| dirs::home_dir())
        .unwrap_or_else(|| PathBuf::from("."));

    cache_dir.join("runes").to_string_lossy().into_owned()
});

pub(crate) fn analyze(
    runefile: &Path,
    color: ColorChoice,
) -> Result<Rune, Error> {
    let src = std::fs::read_to_string(runefile).with_context(|| {
        format!("Unable to read \"{}\"", runefile.display())
    })?;

    let file = SimpleFile::new(runefile.display().to_string(), &src);

    log::debug!("Parsing \"{}\"", runefile.display());

    let mut diags = Diagnostics::new();

    let rune = match runefile.extension().and_then(|ext| ext.to_str()) {
        Some("yaml") | Some("yml") => {
            let parsed = Document::parse(&src)
                .context("Unable to parse the Runefile")?;
            hotg_rune_syntax::analyse_yaml_runefile(&parsed, &mut diags)
        },
        _ => {
            let parsed = hotg_rune_syntax::parse(&src)
                .context("Unable to parse the Runefile")?;
            let rune = hotg_rune_syntax::analyse(&parsed, &mut diags);
            rune
        },
    };

    let mut writer = StandardStream::stdout(color);
    let config = Config::default();

    for diag in &diags {
        codespan_reporting::term::emit(&mut writer, &config, &file, diag)
            .context("Unable to print the diagnostic")?;
    }

    if diags.has_errors() {
        anyhow::bail!("Aborting compilation due to errors.");
    }

    Ok(rune)
}

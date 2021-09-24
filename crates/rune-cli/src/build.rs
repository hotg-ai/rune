use anyhow::{Context, Error};
use codespan_reporting::{
    files::SimpleFile,
    term::{termcolor::StandardStream, Config, termcolor::ColorChoice},
};
use hotg_rune_compiler::{
    BuildContext, Diagnostics, FeatureFlags, Verbosity,
    codegen::RuneVersion,
    compile::{CompilationResult, CompiledBinary},
    hooks::{
        AfterCodegenContext, AfterLoweringContext, AfterParseContext,
        AfterTypeCheckingContext, Continuation,
    },
};
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

    /// Unlock unstable features.
    #[structopt(long)]
    unstable: bool,
    /// (unstable) A path to the Rune repository. Primarily used to patch
    /// dependencies when hacking on Rune locally.
    #[structopt(long, requires = "unstable", parse(from_os_str))]
    rune_repo_dir: Option<PathBuf>,
}

impl Build {
    pub fn execute(self, color: ColorChoice) -> Result<(), Error> {
        let ctx = self.build_context()?;
        let features = self.feature_flags();

        log::debug!(
            "Compiling {} in \"{}\"",
            ctx.name,
            ctx.working_directory.display()
        );

        let dest = self.output.unwrap_or_else(|| {
            ctx.current_directory.join(&ctx.name).with_extension("rune")
        });

        let mut hooks = Hooks::new(dest, color, self.runefile.clone());
        hotg_rune_compiler::build_with_hooks(ctx, features, &mut hooks);

        match hooks.error {
            None => Ok(()),
            Some(e) => Err(e),
        }
    }

    fn build_context(&self) -> Result<BuildContext, Error> {
        let verbosity =
            Verbosity::from_quiet_and_verbose(self.quiet, self.verbose)
                .context(
                    "The --verbose and --quiet flags can't be used together",
                )?;

        let current_directory = self.current_directory()?;
        let name = self.name()?;

        let working_directory = self
            .cache_dir
            .clone()
            .unwrap_or_else(|| Path::new(&*DEFAULT_CACHE_DIR).join(&name));
        let runefile =
            std::fs::read_to_string(&self.runefile).with_context(|| {
                format!("Unable to read \"{}\"", self.runefile.display())
            })?;

        Ok(BuildContext {
            name,
            current_directory,
            runefile,
            verbosity,
            working_directory,
            optimized: !self.debug,
            rune_version: Some(RuneVersion::new(env!("CARGO_PKG_VERSION"))),
        })
    }

    fn feature_flags(&self) -> FeatureFlags {
        let mut features = FeatureFlags::default();

        if self.unstable {
            features.set_rune_repo_dir(self.rune_repo_dir.clone());
        }

        features
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

static DEFAULT_CACHE_DIR: Lazy<String> = Lazy::new(|| {
    let cache_dir = dirs::cache_dir()
        .or_else(|| dirs::home_dir())
        .unwrap_or_else(|| PathBuf::from("."));

    cache_dir.join("runes").to_string_lossy().into_owned()
});

#[derive(Debug)]
struct Hooks {
    dest: PathBuf,
    runefile_path: PathBuf,
    color: ColorChoice,
    error: Option<Error>,
}

impl Hooks {
    fn new(dest: PathBuf, color: ColorChoice, runefile_path: PathBuf) -> Self {
        Hooks {
            dest,
            color,
            runefile_path,
            error: None,
        }
    }

    fn save_binary(&self, binary: &CompiledBinary) -> Result<(), Error> {
        if let Some(parent) = self.dest.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!(
                    "Unable to create the \"{}\" directory",
                    parent.display()
                )
            })?;
        }

        std::fs::write(&self.dest, &binary).with_context(|| {
            format!("Unable to write to \"{}\"", self.dest.display())
        })?;

        log::info!("The Rune was written to \"{}\"", self.dest.display());

        Ok(())
    }

    fn check_diagnostics(
        &mut self,
        diags: &Diagnostics,
        ctx: &BuildContext,
    ) -> Continuation {
        let mut writer = StandardStream::stderr(self.color);
        let config = Config::default();

        let file = SimpleFile::new(
            self.runefile_path.display().to_string(),
            &ctx.runefile,
        );

        for diag in diags {
            match codespan_reporting::term::emit(
                &mut writer,
                &config,
                &file,
                diag,
            )
            .context("Unable to print the diagnostic")
            {
                Ok(_) => {},
                Err(e) => {
                    self.error = Some(e);
                    return Continuation::Halt;
                },
            }
        }

        if diags.has_errors() {
            self.error = Some(Error::msg("There were 1 or more errors"));
            Continuation::Halt
        } else {
            Continuation::Continue
        }
    }
}

impl hotg_rune_compiler::hooks::Hooks for Hooks {
    fn after_type_checking(
        &mut self,
        ctx: &mut dyn AfterTypeCheckingContext,
    ) -> Continuation {
        self.check_diagnostics(&ctx.diagnostics(), &ctx.build_context())
    }

    fn after_parse(&mut self, ctx: &mut dyn AfterParseContext) -> Continuation {
        self.check_diagnostics(&ctx.diagnostics(), &ctx.build_context())
    }

    fn after_lowering(
        &mut self,
        ctx: &mut dyn AfterLoweringContext,
    ) -> Continuation {
        self.check_diagnostics(&ctx.diagnostics(), &ctx.build_context())
    }

    fn after_codegen(
        &mut self,
        ctx: &mut dyn AfterCodegenContext,
    ) -> Continuation {
        self.check_diagnostics(&ctx.diagnostics(), &ctx.build_context())
    }

    fn after_compile(
        &mut self,
        ctx: &mut dyn hotg_rune_compiler::hooks::AfterCompileContext,
    ) -> Continuation {
        let CompilationResult(result) = ctx.take_compilation_result();

        if let Err(err) = result
            .map_err(Error::from)
            .and_then(|c| self.save_binary(&c))
        {
            self.error = Some(err);
        }

        Continuation::Continue
    }
}

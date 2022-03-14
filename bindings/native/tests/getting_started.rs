//! The runner for our `getting_started.md` example.
//!
//! This will parse the `getting_started.md` file then synthesize a `main.c`
//! containing all the C code, plus a `build.sh` script which executes any
//! commands it contains.

use std::{fs::File, io::Write, path::PathBuf, process::Command};

use anyhow::{Context as _, Error};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};
use tracing_subscriber::fmt::format::FmtSpan;

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter("info,getting_started=debug,cbindgen=error")
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_target(false)
        .init();

    if std::env::var("CI").is_ok() {
        // TODO: Fix up linking and stuff
        tracing::warn!(
            "The getting_started.md example doesn't work in CI. Exiting."
        );
        return Ok(());
    }

    tracing::info!("Started");

    let ctx =
        Context::from_env().context("Unable to load the build context")?;

    std::fs::create_dir_all(&ctx.build_dir)
        .context("Unable to create the output folder")?;

    copy_across_native_library(&ctx)
        .context("Unable to compile the native library")?;
    generate_header(&ctx).context("Unable to generate the header file")?;
    let parsed = parse_getting_started(&ctx)
        .context("Unable to parse the getting_started.md file")?;
    generate_project(&ctx, &parsed)
        .context("Unable to generate the project")?;
    run_build_script(&ctx).context("Unable to run the build script")?;

    Ok(())
}

#[tracing::instrument(skip(ctx))]
fn run_build_script(ctx: &Context) -> Result<(), Error> {
    let status = Command::new("sh")
        .arg(ctx.build_script())
        .current_dir(&ctx.build_dir)
        .env("RUNE", &ctx.sine_rune)
        .status()
        .context("Unable to start the shell")?;

    anyhow::ensure!(status.success(), "The build script failed");

    Ok(())
}

#[tracing::instrument(skip(ctx))]
fn parse_getting_started(ctx: &Context) -> Result<Parsed, Error> {
    let getting_started = ctx.crate_dir.join("getting_started.md");
    let src = std::fs::read_to_string(&getting_started)
        .context("Unable to read getting_started.md")?;

    tracing::info!(
        getting_started = %getting_started.display(),
        "Parsing getting_started.md",
    );

    Ok(parse_source_file(&src))
}

#[tracing::instrument(skip(ctx))]
fn generate_header(ctx: &Context) -> Result<(), Error> {
    let rune_h = ctx.build_dir.join("rune.h");
    tracing::info!(header = %rune_h.display(), "Generating header file");

    cbindgen::generate(&ctx.crate_dir)?.write_to_file(rune_h);

    Ok(())
}

#[tracing::instrument(skip(ctx))]
fn copy_across_native_library(ctx: &Context) -> Result<(), Error> {
    let library = ctx
        .workspace_root
        .join("target")
        .join("debug")
        .join("librune_native.a");
    let dest = ctx.build_dir.join("librune.a");

    tracing::debug!(
        library = %library.display(),
        dest = %dest.display(),
        "Copying across the library",
    );
    std::fs::copy(&library, dest)?;

    Ok(())
}

fn parse_source_file(src: &str) -> Parsed {
    let events: Vec<_> = Parser::new(src).collect();
    let mut parsed = Parsed::default();

    let code_blocks = CodeBlocks::new(&events);

    for cb in code_blocks {
        match cb.fence {
            "c" => {
                parsed.c_code.push_str(&cb.lines);
            },
            "console" => {
                let mut buffer = String::new();

                for line in cb.lines.lines() {
                    let line = line.trim();

                    if line.starts_with("$") && !buffer.is_empty() {
                        parsed.commands.push(std::mem::take(&mut buffer));
                    }

                    let line = line.trim_start_matches('$');
                    buffer.push_str(line);
                    buffer.push('\n');
                }

                if !buffer.is_empty() {
                    parsed.commands.push(buffer);
                }
            },
            _ => {},
        }
    }

    parsed
}

#[derive(Debug)]
struct Context {
    workspace_root: PathBuf,
    crate_dir: PathBuf,
    build_dir: PathBuf,
    sine_rune: PathBuf,
}

impl Context {
    #[tracing::instrument("loading_context")]
    fn from_env() -> Result<Self, Error> {
        let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = crate_dir
            .ancestors()
            .find(|p| p.join(".git").exists())
            .context("Unable to determine the workspace root")?
            .to_path_buf();

        let build_dir = crate_dir.join("tests").join("generated");

        let sine_dir = workspace_root.join("examples").join("sine");
        let sine_rune = sine_dir.join("sine.rune");

        if !sine_rune.exists() {
            todo!();
        }

        Ok(Context {
            workspace_root,
            crate_dir,
            build_dir,
            sine_rune,
        })
    }

    fn build_script(&self) -> PathBuf { self.build_dir.join("build.sh") }
}

#[tracing::instrument(skip(ctx, parsed))]
fn generate_project(ctx: &Context, parsed: &Parsed) -> Result<(), Error> {
    let main_c = ctx.build_dir.join("main.c");

    tracing::debug!(filename = %main_c.display(), "Saving the C code");
    std::fs::write(&main_c, parsed.c_code.as_bytes())
        .context("Unable to save main.c")?;

    generate_build_script(&ctx, &parsed.commands)
        .context("Unable to generate build.sh")?;

    Ok(())
}

fn generate_build_script(
    ctx: &Context,
    commands: &[String],
) -> Result<(), Error> {
    let build_script = ctx.build_script();
    let mut f = File::create(&build_script)
        .context("Unable to open build.sh for writing")?;

    writeln!(f, "#!/bin/sh")?;
    writeln!(f, "set -xe")?;

    for command in commands {
        writeln!(f, "{}", command.trim())?;
    }

    Ok(())
}

#[derive(Debug, Default)]
struct Parsed {
    commands: Vec<String>,
    c_code: String,
}

struct CodeBlocks<'ev> {
    events: std::slice::Iter<'ev, Event<'ev>>,
}

impl<'ev> CodeBlocks<'ev> {
    fn new(events: &'ev [Event<'ev>]) -> Self {
        CodeBlocks {
            events: events.iter(),
        }
    }
}

impl<'ev> Iterator for CodeBlocks<'ev> {
    type Item = CodeBlock<'ev>;

    fn next(&mut self) -> Option<Self::Item> {
        let fence = skip_until(&mut self.events, |e| match e {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(f))) => Some(f),
            _ => None,
        })?;

        let mut lines = String::new();

        while let Some(Event::Text(text)) = self.events.next() {
            lines.push_str(text);
        }

        Some(CodeBlock { fence, lines })
    }
}

fn skip_until<I, P, Ret>(iter: &mut I, predicate: P) -> Option<Ret>
where
    I: Iterator,
    P: Fn(&I::Item) -> Option<Ret>,
{
    loop {
        let next = iter.next()?;

        if let Some(mapped) = predicate(&next) {
            return Some(mapped);
        }
    }
}

#[derive(Debug, Clone)]
struct CodeBlock<'src> {
    fence: &'src str,
    lines: String,
}

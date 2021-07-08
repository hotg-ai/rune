use crate::BulkCopy;
use anyhow::{Context as _, Error};
use walkdir::{DirEntry, WalkDir};
use std::{
    ffi::{OsStr, OsString},
    fmt::{self, Debug, Formatter},
    fs::File,
    io::{Seek, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    str::FromStr,
};
use zip::write::{ZipWriter, FileOptions};

#[derive(Debug, structopt::StructOpt)]
pub struct Dist {
    #[structopt(short, long, help = "A list of components to exclude")]
    exclude: Vec<String>,
    /// The components to include. If not provided, all available components
    /// will be used.
    #[structopt(possible_values = Component::POSSIBLE_VALUES)]
    requested_components: Vec<String>,
}

impl Dist {
    pub fn run(self) -> Result<(), Error> {
        log::info!("Generating release artifacts");

        let components = self.components()?;

        let cargo = std::env::var_os("CARGO")
            .unwrap_or_else(|| OsString::from("cargo"));

        let project_root = crate::project_root()?;
        let target_dir = project_root.join("target");
        let dist = target_dir.join("dist");
        let workspace_cargo_toml = project_root.join("Cargo.toml");

        clear_directory(&dist).context("Unable to clear the dist directory")?;

        let ctx = Context {
            cargo,
            project_root,
            target_dir,
            dist,
            workspace_cargo_toml,
        };

        for component in components {
            log::info!("Running \"{}\"", component.name);
            (component.execute)(&ctx)?;
        }

        generate_archive(&ctx).context("Unable to generate the zip archive")?;

        Ok(())
    }

    fn components(&self) -> Result<Vec<Component>, Error> {
        let mut all_components = if self.requested_components.is_empty() {
            Component::POSSIBLE_VALUES
                .iter()
                .map(ToString::to_string)
                .collect()
        } else {
            self.requested_components.clone()
        };

        all_components.retain(|name| !self.exclude.contains(name));

        all_components
            .into_iter()
            .map(|name| Component::from_str(&name))
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Context {
    cargo: OsString,
    project_root: PathBuf,
    target_dir: PathBuf,
    dist: PathBuf,
    workspace_cargo_toml: PathBuf,
}

type ComponentFunc = fn(&Context) -> Result<(), Error>;

pub struct Component {
    name: String,
    execute: Box<dyn Fn(&Context) -> Result<(), Error>>,
    function_name: &'static str,
}

impl Component {
    pub const POSSIBLE_VALUES: &'static [&'static str] =
        &["rune", "ffi", "examples", "strip", "docs"];

    fn new<I, F>(name: I, execute: F) -> Self
    where
        I: Into<String>,
        F: Fn(&Context) -> Result<(), Error> + 'static,
    {
        Component {
            name: name.into(),
            execute: Box::new(execute),
            function_name: std::any::type_name::<F>(),
        }
    }
}

impl Debug for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Component {
            name,
            execute: _,
            function_name,
        } = self;

        f.debug_struct("Component")
            .field("name", name)
            .field("execute", function_name)
            .finish()
    }
}

impl FromStr for Component {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let func = match s {
            "strip" => strip_binaries as ComponentFunc,
            "examples" => compile_example_runes as ComponentFunc,
            "rune" => compile_rune_binary as ComponentFunc,
            "ffi" => generate_ffi_header as ComponentFunc,
            "docs" => copy_docs as ComponentFunc,
            _ => anyhow::bail!(
                "Expected one of \"{}\" but found \"{}\"",
                Component::POSSIBLE_VALUES.join("\", \""),
                s,
            ),
        };

        Ok(Component::new(s, func))
    }
}

fn copy_docs(ctx: &Context) -> Result<(), Error> {
    let Context {
        project_root, dist, ..
    } = ctx;

    std::fs::copy(project_root.join("README.md"), dist.join("README.md"))
        .context("Unable to copy the README across")?;

    BulkCopy::new(&["*.md"])?
        .with_max_depth(1)
        .copy(project_root.join(""), dist)?;

    Ok(())
}

fn strip_binaries(ctx: &Context) -> Result<(), Error> {
    if cfg!(windows) {
        // Windows puts all debug info in a PDB file, so there's nothing to
        // strip.
        return Ok(());
    }

    let dist = &ctx.dist;

    log::debug!("Stripping binaries");

    for entry in dist
        .read_dir()
        .context("Unable to read the dist/ directory")?
    {
        let entry = entry?;
        let path = entry.path();

        if !is_strippable(&path) {
            continue;
        }

        if let Err(e) = strip_binary(&path) {
            log::warn!(
                "Running the `strip` command on \"{}\" failed: {:?}",
                path.display(),
                e,
            );
        }
    }

    Ok(())
}

fn strip_binary(path: &Path) -> Result<(), Error> {
    let mut cmd = Command::new("strip");
    cmd.arg(&path).arg("--strip-debug");
    log::debug!("Executing {:?}", cmd);

    let status = cmd.status().context("Unable to execute `strip`")?;

    anyhow::ensure!(status.success(), "Strip returned a non-zero exit code");

    Ok(())
}

fn is_strippable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    let ext = match path.extension() {
        Some(ext) => ext,
        // It's probably an executable
        None => return true,
    };

    let ext = match ext.to_str() {
        Some(ext) => ext.to_lowercase(),
        // non-ascii extension
        None => return false,
    };

    let whitelist = &["a", "exe", "dll", "so"];

    whitelist.contains(&ext.as_str())
}

fn generate_ffi_header(ctx: &Context) -> Result<(), Error> {
    let Context { dist, .. } = ctx;
    let header = dist.join("rune.h");

    log::debug!("Writing FFI headers to \"{}\"", header.display());

    let mut cmd = Command::new(&ctx.cargo);
    cmd.arg("test")
        .arg("--package=rune-native")
        .arg("--features=c-headers")
        .arg("--")
        .arg("generate_headers")
        .env("RUNE_HEADER_FILE", &header);

    log::debug!("Executing {:?}", cmd);
    let status = cmd
        .status()
        .context("Unable to run `cargo`. Is it installed?")?;

    anyhow::ensure!(status.success(), "Unable to generate the header file.");

    Ok(())
}

fn generate_archive(ctx: &Context) -> Result<(), Error> {
    let Context {
        target_dir, dist, ..
    } = ctx;

    let name = archive_name(target_dir)?;
    log::info!("Writing the release archive to \"{}\"", name.display());

    let f = File::create(&name).with_context(|| {
        format!("Unable to open \"{}\" for writing", name.display())
    })?;

    let mut writer = ZipWriter::new(f);

    for entry in WalkDir::new(dist).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        log::debug!("Adding \"{}\" to the archive", path.display());

        if !entry.file_type().is_file() {
            continue;
        }

        add_entry_to_archive(&mut writer, dist, &entry).with_context(|| {
            format!("Unable to add \"{}\" to the archive", path.display())
        })?;
    }

    writer.finish()?;

    Ok(())
}

fn add_entry_to_archive<W>(
    writer: &mut ZipWriter<W>,
    base: &Path,
    entry: &DirEntry,
) -> Result<(), Error>
where
    W: Write + Seek,
{
    let path = entry.path();
    let name = path.strip_prefix(base)?;
    writer.start_file(name.display().to_string(), FileOptions::default())?;

    let mut reader = File::open(path)?;

    std::io::copy(&mut reader, writer)?;
    writer.flush()?;

    Ok(())
}

fn archive_name(target_dir: &Path) -> Result<PathBuf, Error> {
    let mut cmd = Command::new("rustc");
    cmd.arg("--version").arg("--verbose");

    log::debug!("Executing {:?}", cmd);

    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .context("Unable to invoke cargo")?;

    log::debug!("Output: {:?}", output);

    if !output.status.success() {
        anyhow::bail!("Rustc failed");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let target_triple = stdout
        .lines()
        .filter_map(|line| {
            if line.contains("host") {
                line.split(" ").skip(1).next()
            } else {
                None
            }
        })
        .next()
        .context("Unable to determine the target triple")?;

    let name = format!("rune.{}.zip", target_triple.trim());

    Ok(target_dir.join(name))
}

fn compile_example_runes(ctx: &Context) -> Result<(), Error> {
    let Context {
        cargo,
        project_root,
        dist,
        ..
    } = ctx;

    let example_dir = project_root.join("examples");
    let destination_dir = dist.join("examples");

    let copy = BulkCopy::new(&[
        "**/Runefile.yml",
        "*.tflite",
        "*.csv",
        "*.wav",
        "*.png",
        "*.md",
    ])?
    .with_blacklist(&["**/rune-rs/*"])?;

    for entry in example_dir
        .read_dir()
        .context("Unable to read the examples directory")?
    {
        let dir = entry.context("Unable to read the dir entry")?;
        let runefile = dir.path().join("Runefile.yml");

        if !runefile.exists() {
            continue;
        }

        let name = dir.file_name();
        let example = destination_dir.join(&name);

        log::info!("Compiling the \"{}\" rune", name.to_string_lossy());
        compile_example_rune(cargo, &name, &runefile, &example)?;

        log::info!("Copying example artifacts across");
        copy.copy(dir.path(), example)
            .context("Unable to copy example artifacts across")?;
    }

    Ok(())
}

fn compile_example_rune(
    cargo: &OsStr,
    name: &OsStr,
    runefile: &Path,
    example: &Path,
) -> Result<(), Error> {
    let generated_code = example.join("rust");
    let rune = example.join(&name).with_extension("rune");

    let mut cmd = Command::new(cargo);
    cmd.arg("run")
        .arg("--release")
        .arg("--package=rune-cli")
        .arg("--")
        .arg("build")
        .arg(&runefile)
        .arg("--cache-dir")
        .arg(&generated_code)
        .arg("--output")
        .arg(rune);
    log::debug!("Executing {:?}", cmd);

    let status = cmd.status().context("Unable to run `rune build`")?;
    anyhow::ensure!(status.success(), "Building the rune failed");

    let mut cmd = Command::new(cargo);
    cmd.arg("clean")
        .arg("--manifest-path")
        .arg(generated_code.join("Cargo.toml"));
    log::debug!("Executing {:?}", cmd);

    let status = cmd.status().context("Unable to run `rune build`")?;
    anyhow::ensure!(status.success(), "Building the rune failed");

    Ok(())
}

fn compile_rune_binary(ctx: &Context) -> Result<(), Error> {
    let Context {
        cargo,
        target_dir,
        dist,
        project_root,
        ..
    } = ctx;

    log::info!("Compiling the `rune` binary");

    let mut cmd = Command::new(cargo);
    cmd.arg("build")
        .arg("--package=rune-cli")
        .arg("--release")
        .current_dir(project_root);
    log::debug!("Executing {:?}", cmd);
    let status = cmd.status().context("Unable to invoke `cargo`")?;

    log::debug!("Executing {:?}", cmd);
    anyhow::ensure!(status.success(), "`cargo build` failed");

    BulkCopy::new(&["**/rune", "**/rune.exe", "**/*rune_native*"])?
        .with_max_depth(1)
        .with_blacklist(&["*.d"])?
        .copy(target_dir.join("release"), dist)
        .context(
            "Unable to copy pre-compiled binaries into the dist directory",
        )?;

    Ok(())
}

fn clear_directory<P: AsRef<Path>>(directory: P) -> Result<(), Error> {
    let directory = directory.as_ref();

    match std::fs::remove_dir_all(directory) {
        Ok(_) => {},
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // the directory doesn't exist, nothing to clean
        },
        Err(e) => return Err(e.into()),
    }

    std::fs::create_dir_all(directory)?;

    Ok(())
}

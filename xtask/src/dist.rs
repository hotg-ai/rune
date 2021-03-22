use crate::BulkCopy;
use anyhow::{Context, Error};
use walkdir::{DirEntry, WalkDir};
use std::{
    ffi::OsStr,
    fs::File,
    io::{Seek, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use zip::write::{ZipWriter, FileOptions};

pub fn generate_release_artifacts() -> Result<(), Error> {
    log::info!("Generating release artifacts");

    let cargo =
        std::env::var("CARGO").unwrap_or_else(|_| String::from("cargo"));

    let project_root = crate::project_root()?;
    let target = project_root.join("target");
    let dist = target.join("dist");
    let workspace_cargo_toml = project_root.join("Cargo.toml");

    clear_directory(&dist).context("Unable to clear the dist directory")?;

    compile_rune_binary(&cargo, &workspace_cargo_toml, &dist, &target)
        .context("Unable to compile binaries")?;
    if let Err(e) = strip_binaries(&dist) {
        log::warn!("Unable to strip the binaries: {}", e);
    }
    generate_ffi_header(&project_root, &dist)
        .context("Unable to generate the header file")?;

    compile_example_runes(&cargo, &project_root, &dist)
        .context("Unable to compile example runes")?;

    std::fs::copy(project_root.join("README.md"), dist.join("README.md"))
        .context("Unable to copy the README across")?;

    BulkCopy::new(&["*.md"])?
        .with_max_depth(1)
        .copy(project_root.join(""), &dist)?;

    generate_archive(&dist, &target)
        .context("Unable to generate the zip archive")?;

    Ok(())
}

#[cfg(not(unix))]
fn strip_binaries(dist: &Path) -> Result<(), Error> {
    // Windows puts all debug info in a PDB file, so there's nothing to strip
    Ok(())
}

#[cfg(unix)]
fn strip_binaries(dist: &Path) -> Result<(), Error> {
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

        let mut cmd = Command::new("strip");
        cmd.arg(&path);
        log::debug!("Executing {:?}", cmd);

        let status = cmd
            .current_dir(&dist)
            .status()
            .context("Unable to execute `strip`")?;

        anyhow::ensure!(
            status.success(),
            "The `strip` command finished unsuccessfully"
        );
    }

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

fn generate_ffi_header(project_root: &Path, dist: &Path) -> Result<(), Error> {
    let ffi_dir = project_root.join("ffi");
    let header = dist.join("rune.h");
    log::debug!("Writing FFI headers at \"{}\"", header.display());
    cbindgen::generate(&ffi_dir)?.write_to_file(&header);

    Ok(())
}

fn generate_archive(dist: &Path, target_dir: &Path) -> Result<(), Error> {
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

fn compile_example_runes(
    cargo: &str,
    project_root: &Path,
    dist: &Path,
) -> Result<(), Error> {
    let example_dir = project_root.join("examples");
    let destination_dir = dist.join("examples");

    let copy = BulkCopy::new(&[
        "**/Runefile",
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
        let runefile = dir.path().join("Runefile");

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
    cargo: &str,
    name: &OsStr,
    runefile: &Path,
    example: &Path,
) -> Result<(), Error> {
    let generated_code = example.join("rust");
    let rune = example.join(&name).with_extension("rune");

    let mut cmd = Command::new(cargo);
    cmd.arg("run")
        .arg("--release")
        .arg("--package")
        .arg("rune")
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

fn compile_rune_binary(
    cargo: &str,
    workspace_cargo_toml: &Path,
    dist: &Path,
    target_dir: &Path,
) -> Result<(), Error> {
    log::info!("Compiling the `rune` binary");

    let mut cmd = Command::new(cargo);
    cmd.arg("build")
        .arg(if log::log_enabled!(log::Level::Debug) {
            "--verbose"
        } else {
            "--quiet"
        })
        .arg("--workspace")
        .arg("--release")
        .arg("--manifest-path")
        .arg(&workspace_cargo_toml);
    let status = cmd.status().context("Unable to invoke `cargo`")?;

    log::debug!("Executing {:?}", cmd);
    anyhow::ensure!(status.success(), "`cargo build` failed");

    BulkCopy::new(&[
        "**/rune",
        "**/rune.exe",
        "**/*.a",
        "**/*.so",
        "**/*.dylib",
        "**/*.dll",
    ])?
    .with_max_depth(1)
    .copy(target_dir.join("release"), dist)
    .context("Unable to copy pre-compiled binaries into the dist directory")?;

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

use anyhow::{Context, Error};
use codespan_reporting::{
    files::SimpleFiles,
    term::{
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
};
use rune_codegen::Compilation;
use rune_syntax::Diagnostics;
use std::path::Path;

pub fn build(runefile: impl AsRef<Path>) -> Result<(), Error> {
    let runefile = runefile.as_ref();
    let src = std::fs::read_to_string(runefile).with_context(|| {
        format!("Unable to read \"{}\"", runefile.display())
    })?;
    let mut files = SimpleFiles::new();
    let id = files.add(runefile.display().to_string(), &src);

    let parsed = rune_syntax::parse(&src).unwrap();

    let mut diags = Diagnostics::new();
    let rune = rune_syntax::analyse(id, &parsed, &mut diags);

    let mut writer = StandardStream::stdout(ColorChoice::Auto);
    let config = Config::default();

    for diag in &diags {
        codespan_reporting::term::emit(&mut writer, &config, &files, diag)
            .unwrap();
    }

    if diags.has_errors() {
        anyhow::bail!("Aborting compilation due to errors.");
    }

    let current_directory = runefile.parent().unwrap().to_path_buf();
    let name = current_directory
        .file_name()
        .expect("The directory has a name");

    let working_directory = dirs::home_dir()
        .unwrap()
        .join(".rune")
        .join("runes")
        .join(name);
    let dest = current_directory.join(name).with_extension("rune");

    let compilation = Compilation {
        name: name.to_string_lossy().into_owned(),
        rune,
        current_directory,
        working_directory,
        rune_project_dir: Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf(),
    };
    let blob = rune_codegen::generate(compilation)
        .context("Rune compilation failed")?;

    std::fs::write(&dest, &blob).with_context(|| {
        format!("Unable to write to \"{}\"", dest.display())
    })?;

    Ok(())
}

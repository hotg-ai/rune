use codespan_reporting::{
    files::{Files, SimpleFiles},
    term::{
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
};
use rune_codegen::Compilation;
use rune_syntax::Diagnostics;
use std::path::Path;

pub fn build(runefile: impl AsRef<Path>) {
    let runefile = runefile.as_ref();
    let src = std::fs::read_to_string(runefile).unwrap();

    let mut files = SimpleFiles::new();
    let id = files.add(runefile.display().to_string(), &src);

    let parsed = rune_syntax::parse(&src).unwrap();

    let mut diags = Diagnostics::new();
    let rune = rune_syntax::analyse(0, &parsed, &mut diags);

    let mut writer = StandardStream::stdout(ColorChoice::Auto);
    let config = Config::default();

    for diag in &diags {
        codespan_reporting::term::emit(&mut writer, &config, &files, diag)
            .unwrap();
    }

    if !diags.is_empty() {
        panic!("There were errors");
    }

    let current_directory = runefile.parent().unwrap().to_path_buf();
    let name = current_directory.file_name().unwrap();

    let working_directory = std::env::home_dir()
        .unwrap()
        .join(".rune")
        .join("runes")
        .join(name);
    let dest = current_directory.join(name).with_extension("rune");

    let compilation = Compilation {
        rune,
        current_directory,
        working_directory,
    };
    let blob = rune_codegen::generate(compilation).unwrap();

    std::fs::write(dest, &blob).unwrap();
}

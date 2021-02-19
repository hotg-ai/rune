use anyhow::{Context, Error};
use handlebars::Handlebars;
use rune_syntax::hir::Rune;
use serde::Serialize;
use serde_json::json;
use std::{
    fs::File,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug)]
pub struct Compilation {
    /// The [`Rune`] being compiled to WebAssembly.
    pub rune: Rune,
    /// A directory that can be used for any temporary artifacts.
    pub working_directory: PathBuf,
    /// The directory that all paths (e.g. to models) are resolved relative to.
    pub current_directory: PathBuf,
}

pub fn generate(c: Compilation) -> Result<Vec<u8>, Error> {
    log::info!("Generating");

    let generator = Generator::new(c);

    generator.create_directories()?;
    generator.render()?;
    generator.compile()?;

    todo!()
}

struct Generator {
    hbs: Handlebars<'static>,
    rune: Rune,
    dest: PathBuf,
    current_directory: PathBuf,
}

impl Generator {
    fn new(compilation: Compilation) -> Self {
        let mut hbs = Handlebars::new();

        // Note: all these templates are within our control, so any error here
        // is the developer's fault.
        hbs.register_template_string(
            ".cargo/config",
            include_str!("./boilerplate/cargo_config.hbs"),
        )
        .unwrap();
        hbs.register_template_string(
            "Cargo.toml",
            include_str!("./boilerplate/Cargo.toml.hbs"),
        )
        .unwrap();
        hbs.register_template_string(
            "lib.rs",
            include_str!("./boilerplate/lib.rs.hbs"),
        )
        .unwrap();

        let Compilation {
            rune,
            working_directory,
            current_directory,
        } = compilation;

        Generator {
            hbs,
            rune,
            current_directory,
            dest: working_directory,
        }
    }

    fn create_directories(&self) -> Result<(), Error> {
        create_dir(&self.dest)?;
        create_dir(self.dest.join(".cargo"))?;

        Ok(())
    }

    fn render(&self) -> Result<(), Error> {
        self.render_to(
            self.dest.join(".cargo").join("config"),
            ".cargo/config",
            &json!(null),
        )?;

        self.render_cargo_toml()?;
        self.render_models()?;

        Ok(())
    }

    fn render_cargo_toml(&self) -> Result<(), Error> {
        let mut dependencies = vec![
            json!({ "name": "wee_alloc", "crates_io": "0.4.5" }),
            json!({ "name": "runic_types", "git": "https://github.com/hotg-ai/rune.git" }),
        ];

        for (&id, proc) in &self.rune.proc_blocks {
            let name = self
                .rune
                .names
                .get_name(id)
                .context("Unable to get the PROC_BLOCK's name")?;
            let git_repo = format!("https://github.com/{}.git", proc.path);

            dependencies.push(json!({
                "name": name,
                "git": git_repo,
            }));
        }

        let ctx = json!({ "name": "rune", "dependencies": dependencies });

        self.render_to(self.dest.join("Cargo.toml"), "Cargo.toml", &ctx)?;

        Ok(())
    }

    fn render_to(
        &self,
        dest: impl AsRef<Path>,
        template: &str,
        ctx: &impl Serialize,
    ) -> Result<(), Error> {
        let dest = dest.as_ref();
        let f = File::create(dest).with_context(|| {
            format!("Unable to create \"{}\"", dest.display())
        })?;

        self.hbs
            .render_to_write(template, ctx, f)
            .with_context(|| {
                format!("Unable to generate \"{}\"", dest.display())
            })?;

        Ok(())
    }

    fn render_models(&self) -> Result<(), Error> {
        for (&id, model) in &self.rune.models {
            let name = self
                .rune
                .names
                .get_name(id)
                .context("Unable to get the MODEL's name")?;

            let model_path = self.current_directory.join(&model.model_file);
            let dest = self.dest.join(name).with_extension("tflite");
            std::fs::copy(&model_path, &dest).with_context(|| {
                format!(
                    "Unable to copy \"{}\" to \"{}\"",
                    model_path.display(),
                    dest.display()
                )
            })?;
        }

        Ok(())
    }

    fn compile(&self) -> Result<(), Error> {
        let status = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .arg("--target=wasm32-unknown-unknown")
            .arg("--quiet")
            .current_dir(&self.dest)
            .status()
            .context("Unable to start `cargo`. Is it installed?")?;

        if status.success() {
            Ok(())
        } else {
            Err(Error::msg("Compilation failed"))
        }
    }
}

fn create_dir(path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();
    std::fs::create_dir_all(path)
        .with_context(|| format!("Unable to create \"{}\"", path.display()))
}

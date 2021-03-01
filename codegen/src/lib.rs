use anyhow::{Context as _, Error};
use handlebars::{
    Context, Handlebars, Helper, Output, RenderContext, RenderError,
};
use rune_syntax::{
    ast::{ArgumentValue, Literal, LiteralKind},
    hir::{Rune, Sink, SourceKind},
};
use serde::Serialize;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug)]
pub struct Compilation {
    /// The name of the [`Rune`] being compiled.
    pub name: String,
    /// The [`Rune`] being compiled to WebAssembly.
    pub rune: Rune,
    /// A directory that can be used for any temporary artifacts.
    pub working_directory: PathBuf,
    /// The directory that all paths (e.g. to models) are resolved relative to.
    pub current_directory: PathBuf,
    pub rune_project_dir: PathBuf,
}

pub fn generate(c: Compilation) -> Result<Vec<u8>, Error> {
    log::info!("Generating {}", c.name);

    let generator = Generator::new(c);

    generator.create_directories()?;
    generator.render()?;
    generator.compile()?;

    let wasm = generator
        .dest
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(&generator.name)
        .with_extension("wasm");

    std::fs::read(&wasm)
        .with_context(|| format!("Unable to read \"{}\"", wasm.display()))
}

struct Generator {
    name: String,
    hbs: Handlebars<'static>,
    rune: Rune,
    dest: PathBuf,
    current_directory: PathBuf,
    rune_project_dir: PathBuf,
}

impl Generator {
    fn new(compilation: Compilation) -> Self {
        let mut hbs = Handlebars::new();
        hbs.set_strict_mode(true);

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
        hbs.register_helper("toml", Box::new(to_toml));

        let Compilation {
            name,
            rune,
            working_directory,
            current_directory,
            rune_project_dir,
        } = compilation;

        Generator {
            name,
            hbs,
            rune,
            current_directory,
            rune_project_dir,
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
        self.render_lib_rs()?;

        Ok(())
    }

    fn render_cargo_toml(&self) -> Result<(), Error> {
        let runic_types = self.rune_project_dir.join("runic-types");
        let mut dependencies = vec![
            json!({ "name": "wee_alloc", "deps": { "version": "0.4.5"} }),
            json!({ "name": "runic-types", "deps": {"path": runic_types} }),
        ];

        for proc in self.rune.proc_blocks.values() {
            dependencies.push(dependency_info(proc, &self.rune_project_dir));
        }

        let ctx = json!({ "name": self.name, "dependencies": dependencies });

        self.render_to(self.dest.join("Cargo.toml"), "Cargo.toml", &ctx)?;

        Ok(())
    }

    fn render_lib_rs(&self) -> Result<(), Error> {
        let ctx = json!({
            "models": self.models(),
            "capabilities": self.capabilities(),
            "outputs": self.outputs(),
        });
        self.render_to(self.dest.join("lib.rs"), "lib.rs", &ctx)?;

        Ok(())
    }

    fn models(&self) -> Vec<Value> {
        self.rune
            .models
            .keys()
            .filter_map(|&id| self.rune.names.get_name(id))
            .map(Value::from)
            .collect()
    }

    fn outputs(&self) -> Vec<Value> {
        self.rune
            .sinks
            .values()
            .map(|sink| match sink {
                Sink::Serial => Value::from("SERIAL"),
            })
            .collect()
    }

    fn capabilities(&self) -> Vec<Value> {
        let mut capabilities = Vec::new();

        for (&id, source) in &self.rune.sources {
            if let Some(name) = self.rune.names.get_name(id) {
                let kind = match &source.kind {
                    SourceKind::Random => "RAND",
                    SourceKind::Accelerometer => "ACCEL",
                    SourceKind::Other(name) => name.as_str(),
                };

                capabilities.push(json!({
                    "name": name,
                    "kind": kind,
                    "parameters": source.parameters.iter()
                        .map(|p| (&p.name.value, jsonify_arg_value(&p.value)))
                        .collect::<HashMap<_, _>>(),
                }));
            }
        }

        capabilities
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

fn dependency_info(
    proc: &rune_syntax::hir::ProcBlock,
    rune_project_dir: &Path,
) -> serde_json::Value {
    const BUILTIN_PROC_BLOCKS: &[&str] =
        &["mod360", "modulo", "normalize", "ohv_label"];

    let name = dbg!(proc.name());

    if BUILTIN_PROC_BLOCKS.contains(&name) {
        let path = rune_project_dir.join("proc_blocks").join(name);
        json!({
            "name": name,
            "deps": {"path": path.display().to_string() },
        })
    } else {
        let repo = format!("https://github.com/{}.git", proc.path.base);
        json!({
            "name": name,
            "deps": {"git": repo },
        })
    }
}

#[derive(Debug)]
struct Dependency {
    name: String,
    ty: String,
}

fn create_dir(path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();
    std::fs::create_dir_all(path)
        .with_context(|| format!("Unable to create \"{}\"", path.display()))
}

fn jsonify_arg_value(arg: &ArgumentValue) -> Value {
    match arg {
        ArgumentValue::Literal(Literal {
            kind: LiteralKind::Integer(i),
            ..
        }) => Value::from(*i),
        ArgumentValue::Literal(Literal {
            kind: LiteralKind::Float(f),
            ..
        }) => Value::from(*f),
        ArgumentValue::Literal(Literal {
            kind: LiteralKind::String(s),
            ..
        }) => Value::from(s.as_str()),
        ArgumentValue::List(list) => {
            Value::Array(list.iter().map(|s| Value::from(s.as_str())).collect())
        },
    }
}

fn to_toml(
    h: &Helper<'_, '_>,
    _: &Handlebars<'_>,
    _: &Context,
    _: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or_else(|| RenderError::new("Missing parameter"))?;

    let as_toml = toml::to_string(param.value()).map_err(|e| {
        RenderError::from_error("Unable to serialize as toml", e)
    })?;
    out.write("{")?;
    out.write(as_toml.trim())?;
    out.write("}")?;

    Ok(())
}

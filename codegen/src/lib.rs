use anyhow::{Context as _, Error};
use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};
use heck::CamelCase;
use rune_syntax::{
    ast::{ArgumentValue, Literal, LiteralKind},
    hir::{HirId, Rune, Sink, SourceKind, Type},
};
use serde::Serialize;
use serde_json::{json, Value};
use std::{
    fs::File,
    path::{Path, PathBuf},
    process::Command,
};
use once_cell::sync::Lazy;

static REQUIRED_DEPENDENCIES: Lazy<Vec<Value>> = Lazy::new(|| {
    vec![json!({
        "name": "log",
        "deps": {
            "version": "0.4",
            "features": ["max_level_debug", "release_max_level_info"]
        }
    })]
});

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
    /// The root directory for the `rune` project (used for locating
    /// dependencies).
    pub rune_project_dir: PathBuf,
    /// Generate an optimized build.
    pub optimized: bool,
}

pub fn generate(c: Compilation) -> Result<Vec<u8>, Error> {
    let generator = Generator::new(c);

    generator.create_directories()?;
    generator.render()?;
    generator.compile()?;

    let build_dir = if generator.optimized {
        "release"
    } else {
        "debug"
    };

    let wasm = generator
        .dest
        .join("target")
        .join("wasm32-unknown-unknown")
        .join(build_dir)
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
    optimized: bool,
}

impl Generator {
    fn new(compilation: Compilation) -> Self {
        let mut hbs = Handlebars::new();
        hbs.set_strict_mode(true);
        hbs.register_escape_fn(|s| s.to_string());

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
            optimized,
        } = compilation;

        Generator {
            name,
            hbs,
            rune,
            current_directory,
            rune_project_dir,
            optimized,
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

    fn dependencies(&self) -> Vec<Value> {
        let runic_types = self.rune_project_dir.join("runic-types");
        let mut dependencies = REQUIRED_DEPENDENCIES.clone();

        dependencies.push(
            json!({ "name": "runic-types", "deps": { "path": runic_types } }),
        );

        for proc in self.rune.proc_blocks.values() {
            dependencies.push(dependency_info(proc, &self.rune_project_dir));
        }

        dependencies
    }

    fn render_cargo_toml(&self) -> Result<(), Error> {
        let dependencies = self.dependencies();

        let ctx = json!({ "name": self.name, "dependencies": dependencies });
        self.render_to(self.dest.join("Cargo.toml"), "Cargo.toml", &ctx)?;

        Ok(())
    }

    fn render_lib_rs(&self) -> Result<(), Error> {
        let ctx = json!({
            "models": self.models(),
            "capabilities": self.capabilities(),
            "proc_blocks": self.proc_blocks(),
            "outputs": self.outputs(),
            "pipeline": self.pipeline(),
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
        let mut outputs = Vec::new();

        for (&id, sink) in &self.rune.sinks {
            if let Some(name) = self.rune.names.get_name(id) {
                let type_name = match sink {
                    Sink::Serial => "Serial",
                };

                outputs.push(json!({
                    "name": name,
                    "type": type_name,
                }));
            }
        }

        log::trace!("Outputs: {:?}", outputs);

        outputs
    }

    fn capabilities(&self) -> Vec<Value> {
        let mut capabilities = Vec::new();

        for (&id, source) in &self.rune.sources {
            if let Some(name) = self.rune.names.get_name(id) {
                let type_name = match &source.kind {
                    SourceKind::Random => "runic_types::wasm32::Random",
                    SourceKind::Accelerometer => {
                        "runic_types::wasm32::Accelerometer"
                    },
                    SourceKind::Sound => "runic_types::wasm32::Sound",
                    SourceKind::Image => "runic_types::wasm32::Image",
                    SourceKind::Other(name) => name.as_str(),
                };

                capabilities.push(json!({
                    "name": name,
                    "type": type_name,
                    "parameters": source.parameters.iter()
                        .map(|(name, value)| json!({
                            "name": name,
                            "value": rust_literal(&value)
                        }))
                        .collect::<Vec<_>>(),
                }));
            }
        }

        log::trace!("Capabilities: {:?}", capabilities);

        capabilities
    }

    fn proc_blocks(&self) -> Vec<Value> {
        let mut blocks = Vec::new();

        for (&id, proc_block) in &self.rune.proc_blocks {
            if let Some(name) = self.rune.names.get_name(id) {
                let module_name = proc_block.name();
                let type_name =
                    format!("{}::{}", module_name, module_name.to_camel_case());

                let parameters = proc_block
                    .parameters
                    .iter()
                    .map(|(name, value)| {
                        json!({
                            "name": name,
                            "value": rust_literal(&value)
                        })
                    })
                    .collect::<Vec<_>>();

                blocks.push(json!({
                    "name": name,
                    "type": type_name,
                    "parameters": parameters,
                }));
            }
        }

        blocks
    }

    fn pipeline(&self) -> Vec<Value> {
        #[derive(serde::Serialize)]
        struct Stage<'a> {
            name: &'a str,
            first: bool,
            last: bool,
            output_type: Option<String>,
        }

        let pipeline = self
            .rune
            .pipelines
            .values()
            .next()
            .expect("There should be at least one pipeline");

        let mut stages = Vec::new();

        for node in pipeline.iter() {
            let name = self
                .rune
                .names
                .get_name(node.id())
                .expect("All pipeline nodes have names");

            let output_type =
                node.output_type().and_then(|t| self.rust_type_name(t));

            stages.push(Stage {
                name,
                output_type,
                first: false,
                last: false,
            });
        }

        assert!(stages.len() >= 2);
        stages.first_mut().unwrap().first = true;
        stages.last_mut().unwrap().last = true;

        let pipelines: Vec<_> = stages
            .into_iter()
            .map(|s| serde_json::to_value(&s).unwrap())
            .collect();

        log::trace!("Pipelines: {:?}", pipelines);

        pipelines
    }

    fn rust_type_name(&self, id: HirId) -> Option<String> {
        let ty = self.rune.types.get(&id)?;

        match ty {
            Type::Primitive(p) => Some(p.rust_name().to_string()),
            Type::Buffer {
                underlying_type,
                dimensions,
            } => self.rust_array_type_name(*underlying_type, dimensions),
            Type::Any | Type::Unknown => None,
        }
    }

    fn rust_array_type_name(
        &self,
        underlying_type: HirId,
        dimensions: &[usize],
    ) -> Option<String> {
        match dimensions.split_first() {
            Some((dim, rest)) => {
                let inner = self.rust_array_type_name(underlying_type, rest)?;

                Some(format!("[{}; {}]", inner, dim))
            },
            None => self.rust_type_name(underlying_type),
        }
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
        let mut cmd = Command::new("cargo");
        cmd.arg("build")
            .arg("--target=wasm32-unknown-unknown")
            .arg("--quiet")
            .current_dir(&self.dest);

        if self.optimized {
            cmd.arg("--release");
        }

        log::debug!("Executing {:?}", cmd);
        let status = cmd
            .status()
            .context("Unable to start `cargo`. Is it installed?")?;

        if status.success() {
            Ok(())
        } else {
            Err(Error::msg("Compilation failed"))
        }
    }
}

fn rust_literal(arg: &ArgumentValue) -> String {
    match arg {
        ArgumentValue::Literal(Literal {
            kind: LiteralKind::Integer(i),
            ..
        }) => i.to_string(),
        ArgumentValue::Literal(Literal {
            kind: LiteralKind::Float(f),
            ..
        }) => format!("{:.1}", f),
        ArgumentValue::Literal(Literal {
            kind: LiteralKind::String(s),
            ..
        }) => format!("{:?}", s),
        ArgumentValue::List(items) => format!("{:?}", items),
    }
}

fn dependency_info(
    proc: &rune_syntax::hir::ProcBlock,
    rune_project_dir: &Path,
) -> serde_json::Value {
    let name = proc.name();

    if is_builtin(&proc.path) {
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

fn is_builtin(path: &rune_syntax::ast::Path) -> bool {
    path.base == "hotg-ai/rune"
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

    let toml = as_inline_toml(param.value());
    out.write(&toml)?;

    Ok(())
}

fn as_inline_toml(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("{:?}", s),
        Value::Array(arr) => {
            let mut buffer = String::new();
            buffer.push_str("[");
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    buffer.push_str(", ");
                }

                let item = as_inline_toml(item);
                buffer.push_str(&item);
            }
            buffer.push_str("]");

            buffer
        },
        Value::Object(obj) => {
            let mut buffer = String::new();
            buffer.push_str("{ ");
            for (i, (key, value)) in obj.iter().enumerate() {
                if i > 0 {
                    buffer.push_str(", ");
                }

                buffer.push_str(key);
                buffer.push_str(" = ");
                let value = as_inline_toml(value);
                buffer.push_str(&value);
            }
            buffer.push_str(" }");

            buffer
        },
    }
}

#[cfg(test)]
mod tests {
    use rune_syntax::ast::Path;

    use super::*;

    #[test]
    fn detect_builtin_proc_blocks() {
        let inputs = vec![
            ("hotg-ai/rune#proc_blocks/normalize", true),
            ("https://github.com/hotg-ai/rune", false),
            ("hotg-ai/rune", true),
            ("hotg-ai/rune@latest", true),
        ];

        for (path, should_be) in inputs {
            let path: Path = path.parse().unwrap();
            let got = is_builtin(&path);

            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn json_object_to_inline_table() {
        let object = json!({
            "default-features": false,
            "version": "1.7.0",
            "features": ["a", "b"]
        });

        let got = as_inline_toml(&object);
        assert_eq!(
            got,
            r#"{ default-features = false, features = ["a", "b"], version = "1.7.0" }"#
        );

        #[derive(serde::Deserialize)]
        struct Document {
            temp: Temp,
        }
        #[derive(serde::Deserialize)]
        struct Temp {
            foo: Value,
        }

        let src = format!("[temp]\nfoo = {}", got);
        let deserialized: Document = toml::from_str(&src).unwrap();
        assert_eq!(deserialized.temp.foo, object);
    }
}

use std::{
    error::Error,
    io::{Cursor, Seek, Write},
    sync::Arc,
};

use crate::{
    im::{OrdMap, Vector},
    parse::{
        Argument, Path, ResourceDeclaration, ResourceName, ResourceOrString,
        Stage,
    },
};
use indexmap::IndexMap;
use zip::{result::ZipResult, write::FileOptions, ZipWriter};

use crate::{
    parse::{DocumentV1, Frontend},
    Text,
};

#[salsa::query_group(CodegenStorage)]
pub trait Codegen: Frontend {
    /// Create a self-contained [`DocumentV1`] with all resources resolved and
    /// pointing to "local" (according to the ZIP archive generated by
    /// [`Codegen::rune_archive()`]) files.
    #[salsa::dependencies]
    fn self_contained_runefile(
        &self,
    ) -> Result<Arc<DocumentV1>, Arc<dyn Error>>;

    #[salsa::dependencies]
    fn rune_archive(&self) -> Result<Vector<u8>, Arc<dyn Error>>;
}

#[tracing::instrument(skip(db))]
fn self_contained_runefile(
    db: &dyn Codegen,
) -> Result<Arc<DocumentV1>, Arc<dyn Error>> {
    let mut doc = db.parse()?;

    let d = Arc::make_mut(&mut doc);
    patch_arguments(db, &mut d.pipeline)?;
    patch_paths(&mut d.pipeline)?;
    patch_resources(&mut d.resources)?;

    Ok(doc)
}

#[tracing::instrument(skip(db, stages))]
fn patch_arguments(
    db: &dyn Codegen,
    stages: &mut IndexMap<String, Stage>,
) -> Result<(), Arc<dyn Error>> {
    for (stage_name, stage) in stages {
        for (arg_name, arg_value) in stage.args_mut() {
            if let Argument(ResourceOrString::Resource(ResourceName(res))) =
                arg_value
            {
                let value = db.resource_value(res.as_str().into())?;
                tracing::debug!(
                    stage=%stage_name,
                    arg=%arg_name,
                    value_len=value.len(),
                    "Patched an argument",
                );
                let s = std::str::from_utf8(&value)
                    .map_err(|e| Arc::new(e) as Arc<dyn Error>)?;
                *arg_value = Argument(ResourceOrString::String(s.to_string()));
            }
        }
    }

    Ok(())
}

#[tracing::instrument(skip(stages))]
fn patch_paths(
    stages: &mut IndexMap<String, Stage>,
) -> Result<(), Arc<dyn Error>> {
    for (name, stage) in stages {
        match stage {
            Stage::Model(m) => {
                let path = Path::FileSystem(format!("models/{name}"));
                tracing::debug!(new=%path, old=%m.model, "Patching model path");
                m.model = path;
            },
            Stage::ProcBlock(p) => {
                let path = Path::FileSystem(format!("proc_blocks/{name}"));
                tracing::debug!(new=%path, old=%p.proc_block, "Patching proc-block path");
                p.proc_block = path;
            },
            Stage::Capability(_) | Stage::Out(_) => {},
        }
    }

    Ok(())
}

#[tracing::instrument(skip(resources))]
fn patch_resources(
    resources: &mut IndexMap<String, ResourceDeclaration>,
) -> Result<(), Arc<dyn Error>> {
    for (name, decl) in resources {
        decl.inline = None;
        let path = format!("resources/{name}");
        tracing::debug!(?path, "Patched resource");
        decl.path = Some(path);
    }

    Ok(())
}

#[tracing::instrument(skip(db))]
fn rune_archive(db: &dyn Codegen) -> Result<Vector<u8>, Arc<dyn Error>> {
    let runefile = db.self_contained_runefile()?;
    let runefile = serde_yaml::to_string(&*runefile)
        .expect("Serializing to YAML should never fail");
    let proc_blocks = db.proc_blocks()?;
    let models = db.model_files()?;
    let resources = db.resource_values()?;

    generate_archive(&runefile, &proc_blocks, &models, &resources)
        .map(Vector::from)
        .map_err(|e| Arc::new(e) as Arc<dyn Error>)
}

fn generate_archive(
    runefile: &str,
    proc_blocks: &OrdMap<Text, Vector<u8>>,
    models: &OrdMap<Text, Vector<u8>>,
    resources: &OrdMap<Text, Vector<u8>>,
) -> ZipResult<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut writer = ZipWriter::new(Cursor::new(&mut buffer));

    writer.start_file("Runefile.yml", FileOptions::default())?;
    writer.write_all(runefile.as_bytes())?;

    for (name, data) in resources {
        write_to_directory(&mut writer, "resources", &name, data)?;
    }

    for (name, data) in proc_blocks {
        write_to_directory(&mut writer, "proc_blocks", &name, data)?;
    }

    for (name, data) in models {
        write_to_directory(&mut writer, "models", &name, data)?;
    }

    writer.finish()?;
    drop(writer);
    Ok(buffer)
}

#[tracing::instrument(skip(writer, data))]
fn write_to_directory(
    writer: &mut ZipWriter<impl Write + Seek>,
    directory: &str,
    name: &str,
    data: &[u8],
) -> ZipResult<()> {
    let path = format!("{directory}/{name}");
    tracing::debug!(%directory, %path, bytes = %data.len(), "Writing to file");
    writer.start_file(&path, FileOptions::default())?;
    writer.write_all(data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{path::Path};
    use tracing_test::traced_test;
    use uriparse::{Scheme, URI};
    use zip::ZipArchive;

    use super::*;
    use crate::{
        parse::Frontend, parse::FrontendStorage, BuildConfig, Environment,
        EnvironmentStorage, FileSystem, ReadError,
    };

    #[derive(Default)]
    #[salsa::database(FrontendStorage, EnvironmentStorage, CodegenStorage)]
    struct Database {
        storage: salsa::Storage<Self>,
    }

    impl salsa::Database for Database {}

    // The parsing process requires you to load proc-blocks and read files. You
    // can satisfy these dependencies by implementing the corresponding traits.

    impl FileSystem for Database {
        fn read(&self, path: &URI<'_>) -> Result<Vector<u8>, ReadError> {
            match path.scheme() {
                Scheme::File => Ok(Vector::default()),
                Scheme::Unregistered(s) if s.as_str() == "wapm" => {
                    Ok(Vector::default())
                },
                _ => unimplemented!(),
            }
        }
    }

    #[test]
    #[traced_test]
    fn smoke_test() {
        let project_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        let sine_dir = project_root.join("examples").join("sine");
        let runefile =
            std::fs::read_to_string(sine_dir.join("Runefile.yml")).unwrap();

        let mut db = Database::default();
        db.set_config(BuildConfig {
            current_directory: sine_dir.clone(),
        });
        db.set_src(runefile.into());

        let zune = db.rune_archive().unwrap();

        let mut reader = ZipArchive::new(Cursor::new(&*zune)).unwrap();

        let mut entries: Vec<_> =
            reader.file_names().map(String::from).collect();
        entries.sort();
        assert_eq!(
            entries,
            &["Runefile.yml", "models/sine", "proc_blocks/mod360",]
        );

        let expected = r#"
            version: 1
            image: runicos/base
            pipeline:
                rand:
                    capability: RAW
                    outputs:
                    - type: F32
                      dimensions:
                        - 1
                        - 1
                    args:
                        length: "4"
                mod360:
                    proc-block: proc_blocks/mod360
                    inputs:
                    - rand
                    outputs:
                    - type: F32
                      dimensions:
                        - 1
                        - 1
                    args:
                        modulus: "360"
                sine:
                    model: models/sine
                    inputs:
                    - mod360
                    outputs:
                    - type: F32
                      dimensions:
                        - 1
                        - 1
                serial:
                    out: serial
                    inputs:
                    - sine
            resources: {}"#;

        let f = reader.by_name("Runefile.yml").unwrap();
        let actual: serde_yaml::Value = serde_yaml::from_reader(f).unwrap();
        let expected: serde_yaml::Value =
            serde_yaml::from_str(expected).unwrap();

        assert_eq!(actual, expected);
    }
}

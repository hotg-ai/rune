use std::{
    error::Error,
    io::{Cursor, Seek, Write},
    sync::Arc,
};

use crate::im::{OrdMap, Vector};
use zip::{result::ZipResult, write::FileOptions, ZipWriter};

use crate::{
    parse::{DocumentV1, Frontend},
    Text,
};

#[salsa::query_group(CodegenStorage)]
pub trait Codegen: Frontend {
    /// Create a self-contained [`DocumentV1`] with all resources resolved and
    ///
    fn self_contained_runefile(&self) -> Arc<DocumentV1>;

    #[salsa::dependencies]
    fn rune_archive(&self) -> Result<Vector<u8>, Arc<dyn Error>>;
}

#[tracing::instrument(skip(db))]
fn self_contained_runefile(db: &dyn Codegen) -> Arc<DocumentV1> {
    db.parse().unwrap()
}

#[tracing::instrument(skip(db))]
fn rune_archive(db: &dyn Codegen) -> Result<Vector<u8>, Arc<dyn Error>> {
    let runefile = db.self_contained_runefile();
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
    writer.start_file(&path, FileOptions::default())?;
    writer.write_all(data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;
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

        let reader = ZipArchive::new(Cursor::new(&*zune)).unwrap();

        let mut entries: Vec<_> = reader.file_names().collect();
        entries.sort();
        assert_eq!(
            entries,
            &["Runefile.yml", "models/sine", "proc_blocks/mod360",]
        );
    }
}

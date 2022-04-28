use std::{error::Error, sync::Arc};

use im::{OrdMap, Vector};
use uriparse::{URIBuilder, URIError, URI};

use crate::{
    parse::{
        Document, DocumentV1, ItemType, ModelStage, NotFound, ParseFailed,
        Path, ProcBlockStage, ResourceDeclaration, Stage, WrongItemType,
    },
    BuildConfig, Environment, FileSystem, Text,
};

/// The Rune compiler's YAML frontend.
///
/// # Examples
///
/// You will typically use the [`Frontend`] by first setting the `Runefile.yml`
/// source code ([`Frontend::set_src()`]) and then parse the document with
/// [`Frontend::parse()`].
///
/// ```rust
/// use hotg_rune_compiler_2::{
///     parse::{Frontend, FrontendStorage},
///     EnvironmentStorage, FileSystem, ReadError, parse::Path,
/// };
/// use uriparse::URI;
/// # use im::Vector;
///
/// // First, you need to create a database which can hold Salsa's state
///
/// #[derive(Default)]
/// #[salsa::database(FrontendStorage, EnvironmentStorage)]
/// struct Database {
///     storage: salsa::Storage<Self>,
/// }
///
/// impl salsa::Database for Database {}
///
/// // The parsing process requires you to load proc-blocks and read files. You
/// // can satisfy these dependencies by implementing the corresponding traits.
///
/// impl FileSystem for Database {
///     fn read(&self, path: &URI<'_>) -> Result<Vector<u8>, ReadError> {
///         todo!();
///     }
/// }
///
/// // Let's hard-code a YAML document to parse
/// let runefile = r#"
///     version: 1
///     image: runicos/base
///     pipeline:
///       input:
///         capability: RAND
///         outputs:
///         - type: f32
///           dimensions: [1]
/// "#;
///
/// // Instantiate the database
/// let mut db = Database::default();
///
/// // Set the source
/// db.set_src(runefile.into());
///
/// // And parse it
/// let doc = db.parse().unwrap();
/// ```
#[salsa::query_group(FrontendStorage)]
pub trait Frontend: Environment + FileSystem {
    /// The YAML document being parsed.
    #[salsa::input]
    fn src(&self) -> Text;

    /// Parse the [`Frontend::src()`] into a [`DocumentV1`].
    #[salsa::dependencies]
    fn parse(&self) -> Result<Arc<DocumentV1>, Arc<dyn Error>>;

    /// A low-level query for parsing a YAML file.
    #[salsa::dependencies]
    fn parse_runefile(&self, src: Text) -> Result<Arc<Document>, ParseFailed>;

    /// Get a resource's value by either returning the value as-is (for
    /// `inline` resources) or reading the file from the filesystem.
    #[salsa::dependencies]
    fn resource_value(&self, name: Text) -> Result<Vector<u8>, Arc<dyn Error>>;

    #[salsa::dependencies]
    fn resource_values(
        &self,
    ) -> Result<OrdMap<Text, Vector<u8>>, Arc<dyn Error>>;

    #[salsa::dependencies]
    fn proc_block(&self, name: Text) -> Result<Vector<u8>, Arc<dyn Error>>;

    /// Get the binary data for each proc-block in this Rune, ignoring any which
    /// may have failed to load.
    #[salsa::dependencies]
    fn proc_blocks(&self) -> Result<OrdMap<Text, Vector<u8>>, Arc<dyn Error>>;

    #[salsa::dependencies]
    fn model_file(&self, name: Text) -> Result<Vector<u8>, Arc<dyn Error>>;
}

#[tracing::instrument(skip(src), err)]
fn parse_runefile(
    _: &dyn Frontend,
    src: Text,
) -> Result<Arc<Document>, ParseFailed> {
    Document::parse(&src)
        .map(Arc::new)
        .map_err(|e| ParseFailed { error: Arc::new(e) })
}

#[tracing::instrument(skip(db))]
fn parse(db: &dyn Frontend) -> Result<Arc<DocumentV1>, Arc<dyn Error>> {
    db.parse_runefile(db.src())
        .map(|d| Arc::new(Document::clone(&d).to_v1()))
        .map_err(|e| Arc::new(e) as Arc<dyn Error>)
}

#[tracing::instrument(skip(db), err)]
fn resource_value(
    db: &dyn Frontend,
    name: Text,
) -> Result<Vector<u8>, Arc<dyn Error>> {
    let doc = db.parse()?;

    let ResourceDeclaration { inline, path, .. } =
        doc.resources.get(name.as_str()).ok_or_else(move || {
            Arc::new(NotFound {
                name,
                item_type: ItemType::Resource,
            }) as Arc<dyn Error>
        })?;

    match (inline, path) {
        (Some(inline), None) => Ok(inline.as_bytes().into()),
        (None, Some(path)) => {
            let path = Path::FileSystem(path.clone());
            read(db, &path)
        },
        (Some(_), Some(_)) => todo!(),
        (None, None) => todo!(),
    }
}

#[tracing::instrument(skip(db))]
fn proc_blocks(
    db: &dyn Frontend,
) -> Result<OrdMap<Text, Vector<u8>>, Arc<dyn Error>> {
    let doc = db.parse()?;

    let mut proc_blocks = OrdMap::new();

    for (name, stage) in &doc.pipeline {
        if let Stage::ProcBlock(_) = stage {
            let binary = db.proc_block(name.into())?;
            proc_blocks.insert(Text::from(name), binary);
        }
    }

    Ok(proc_blocks)
}

#[tracing::instrument(skip(db), err)]
fn proc_block(
    db: &dyn Frontend,
    name: Text,
) -> Result<Vector<u8>, Arc<dyn Error>> {
    let doc = db.parse()?;

    let stage = doc
        .pipeline
        .get(name.as_str())
        .ok_or_else(|| NotFound {
            name,
            item_type: ItemType::ProcBlock,
        })
        .map_err(|e| Arc::new(e) as Arc<dyn Error>)?;

    if let Stage::ProcBlock(ProcBlockStage { proc_block, .. }) = stage {
        read(db, &proc_block)
    } else {
        todo!()
    }
}

#[tracing::instrument(skip(db), err)]
fn resource_values(
    db: &dyn Frontend,
) -> Result<OrdMap<Text, Vector<u8>>, Arc<dyn Error>> {
    let doc = db.parse()?;

    doc.resources
        .keys()
        .map(Text::from)
        .map(|name| {
            let value = db.resource_value(name.clone())?;
            Ok((name, value))
        })
        .collect()
}

#[tracing::instrument(skip(db), err)]
fn model_file(
    db: &dyn Frontend,
    name: Text,
) -> Result<Vector<u8>, Arc<dyn Error>> {
    let doc = db.parse()?;

    let stage = doc
        .pipeline
        .get(name.as_str())
        .ok_or_else(|| NotFound {
            name: name.clone(),
            item_type: ItemType::Model,
        })
        .map_err(|e| Arc::new(e) as Arc<dyn Error>)?;

    let err = move |actual: ItemType| -> Result<Vector<u8>, Arc<dyn Error>> {
        let e = WrongItemType {
            expected: ItemType::Model,
            actual,
            name,
        };
        Err(Arc::new(e) as Arc<dyn Error>)
    };

    match stage {
        Stage::Model(ModelStage { model, .. }) => read(db, &model),
        Stage::Capability(_) => err(ItemType::Input),
        Stage::ProcBlock(_) => err(ItemType::ProcBlock),
        Stage::Out(_) => err(ItemType::Output),
    }
}

fn read(db: &dyn Frontend, path: &Path) -> Result<Vector<u8>, Arc<dyn Error>> {
    file_uri(db, path)
        .map_err(|e| Arc::new(e) as Arc<dyn Error>)
        .and_then(|uri| {
            db.read(&uri).map_err(|e| Arc::new(e) as Arc<dyn Error>)
        })
}

fn file_uri(db: &dyn Frontend, path: &Path) -> Result<URI<'static>, URIError> {
    match path {
        Path::Uri(u) => Ok(u.to_owned()),
        Path::FileSystem(path) => {
            let BuildConfig {
                current_directory, ..
            } = db.config();
            let full_path = current_directory
                .join(path)
                .display()
                .to_string()
                .replace('\\', "/");
            let path = uriparse::Path::try_from(full_path.as_str())?;
            Ok(URIBuilder::new()
                .with_scheme(uriparse::Scheme::File)
                .with_path(path)
                .build()?
                .into_owned())
        },
    }
}

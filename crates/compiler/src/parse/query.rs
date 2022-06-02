use std::{collections::BTreeMap, sync::Arc};

use uriparse::{URIBuilder, URIError, URI};

use crate::{
    asset_loader::AssetLoader,
    im::{OrdMap, Vector},
    parse::{
        CapabilityStage, Document, DocumentV1, ItemType, ModelStage, NotFound,
        ParseFailed, Path, ProcBlockStage, ResourceDeclaration, Stage,
        WellKnownPath, WrongItemType,
    },
    BuildConfig, Environment, Text,
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
/// use hotg_rune_compiler::{
///     parse::{Frontend, FrontendStorage},
///     asset_loader::{AssetLoader, ReadError},
///     parse::Path, EnvironmentStorage, im::Vector,
/// };
/// use uriparse::URI;
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
/// // The parsing process requires you to load proc-blocks and other assets.
/// // You can satisfy these dependencies by implementing the corresponding
/// // traits.
///
/// impl AssetLoader for Database {
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
pub trait Frontend: Environment + AssetLoader {
    /// The YAML document being parsed.
    #[salsa::input]
    fn src(&self) -> Text;

    /// Parse the [`Frontend::src()`] into a [`DocumentV1`].
    #[salsa::dependencies]
    fn parse(&self) -> Result<Arc<DocumentV1>, crate::Error>;

    /// A low-level query for parsing a YAML file.
    #[salsa::dependencies]
    fn parse_runefile(&self, src: Text) -> Result<Arc<Document>, ParseFailed>;

    /// Get a resource's value by either returning the value as-is (for
    /// `inline` resources) or reading the file from the filesystem.
    #[salsa::dependencies]
    fn resource_value(&self, name: Text) -> Result<Vector<u8>, crate::Error>;

    #[salsa::dependencies]
    fn resource_values(&self)
        -> Result<OrdMap<Text, Vector<u8>>, crate::Error>;

    #[salsa::dependencies]
    fn proc_block(&self, name: Text) -> Result<Vector<u8>, crate::Error>;

    /// Get the binary data for each proc-block in this Rune, ignoring any which
    /// may have failed to load.
    #[salsa::dependencies]
    fn proc_blocks(&self) -> Result<OrdMap<Text, Vector<u8>>, crate::Error>;

    #[salsa::dependencies]
    fn model_file(&self, name: Text) -> Result<Vector<u8>, crate::Error>;

    #[salsa::dependencies]
    fn model_files(&self) -> Result<OrdMap<Text, Vector<u8>>, crate::Error>;
}

#[tracing::instrument(level = "debug", skip(src), err)]
fn parse_runefile(
    _: &dyn Frontend,
    src: Text,
) -> Result<Arc<Document>, ParseFailed> {
    Document::parse(&src)
        .map(Arc::new)
        .map_err(|e| ParseFailed { error: Arc::new(e) })
}

#[tracing::instrument(level = "debug", skip(db), err)]
fn parse(db: &dyn Frontend) -> Result<Arc<DocumentV1>, crate::Error> {
    db.parse_runefile(db.src())
        .map(|d| Arc::new(Document::clone(&d).to_v1()))
        .map_err(|e| Arc::new(e) as crate::Error)
}

#[tracing::instrument(level = "debug", skip(db), err)]
fn resource_value(
    db: &dyn Frontend,
    name: Text,
) -> Result<Vector<u8>, crate::Error> {
    let doc = db.parse()?;

    let ResourceDeclaration { inline, path, .. } =
        doc.resources.get(name.as_str()).ok_or_else(move || {
            Arc::new(NotFound {
                name,
                item_type: ItemType::Resource,
            }) as crate::Error
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

#[tracing::instrument(level = "debug", skip(db))]
fn proc_blocks(
    db: &dyn Frontend,
) -> Result<OrdMap<Text, Vector<u8>>, crate::Error> {
    let doc = db.parse()?;

    let mut proc_blocks = BTreeMap::default();

    for (name, stage) in &doc.pipeline {
        match stage {
            Stage::Capability(_) | Stage::ProcBlock(_) => {
                let binary = db.proc_block(name.into())?;
                proc_blocks.insert(Text::from(name), binary);
            },
            _ => {},
        }
    }

    Ok(proc_blocks.into())
}

#[tracing::instrument(level = "debug", skip(db), err)]
fn proc_block(
    db: &dyn Frontend,
    name: Text,
) -> Result<Vector<u8>, crate::Error> {
    let doc = db.parse()?;

    let stage = doc
        .pipeline
        .get(name.as_str())
        .ok_or_else(|| NotFound {
            name,
            item_type: ItemType::ProcBlock,
        })
        .map_err(|e| Arc::new(e) as crate::Error)?;

    match stage {
        Stage::Capability(CapabilityStage {
            capability: path, ..
        })
        | Stage::ProcBlock(ProcBlockStage {
            proc_block: path, ..
        }) => read(db, path),
        _ => todo!(),
    }
}

#[tracing::instrument(level = "debug", skip(db), err)]
fn resource_values(
    db: &dyn Frontend,
) -> Result<OrdMap<Text, Vector<u8>>, crate::Error> {
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

#[tracing::instrument(level = "debug", skip(db), err)]
fn model_file(
    db: &dyn Frontend,
    name: Text,
) -> Result<Vector<u8>, crate::Error> {
    let doc = db.parse()?;

    let stage = doc
        .pipeline
        .get(name.as_str())
        .ok_or_else(|| NotFound {
            name: name.clone(),
            item_type: ItemType::Model,
        })
        .map_err(|e| Arc::new(e) as crate::Error)?;

    let err = move |actual: ItemType| -> Result<Vector<u8>, crate::Error> {
        let e = WrongItemType {
            expected: ItemType::Model,
            actual,
            name,
        };
        Err(Arc::new(e) as crate::Error)
    };

    match stage {
        Stage::Model(ModelStage { model, .. }) => read(db, &model),
        Stage::Capability(_) => err(ItemType::Input),
        Stage::ProcBlock(_) => err(ItemType::ProcBlock),
        Stage::Out(_) => err(ItemType::Output),
    }
}

#[tracing::instrument(level = "debug", skip(db))]
fn model_files(
    db: &dyn Frontend,
) -> Result<OrdMap<Text, Vector<u8>>, crate::Error> {
    let doc = db.parse()?;

    let mut models = BTreeMap::default();

    for (name, stage) in &doc.pipeline {
        if let Stage::Model(_) = stage {
            let binary = db.model_file(name.into())?;
            models.insert(Text::from(name), binary);
        }
    }

    Ok(models.into())
}

fn read(db: &dyn Frontend, path: &Path) -> Result<Vector<u8>, crate::Error> {
    file_uri(db, path)
        .map_err(|e| Arc::new(e) as crate::Error)
        .and_then(|uri| db.read(&uri).map_err(|e| Arc::new(e) as crate::Error))
}

fn file_uri(db: &dyn Frontend, path: &Path) -> Result<URI<'static>, URIError> {
    match path {
        Path::WellKnown(w) => Ok(wapm_uri(*w)),
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

fn wapm_uri(w: WellKnownPath) -> URI<'static> {
    let uri = match w {
        WellKnownPath::Accel => {
            "wapm:///hotg-ai/accelerometer_input?version=0.12.0"
        },
        WellKnownPath::Image => "wapm:///hotg-ai/image_input?version=0.12.0",
        WellKnownPath::Raw => "wapm:///hotg-ai/tensor_input?version=0.12.0",
        WellKnownPath::Sound => "wapm:///hotg-ai/sound_input?version=0.12.0",
    };

    uri.try_into().expect("Should never fail")
}

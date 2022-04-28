use std::{error::Error, sync::Arc};

use im::{OrdMap, Vector};

use crate::{
    parse::{
        Document, DocumentV1, NoSuchProcBlock, NoSuchResource, ParseFailed,
        ProcBlockStage, ResourceDeclaration, Stage,
    },
    proc_blocks::ProcBlockRegistry,
    Environment, FileSystem, Text,
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
///     EnvironmentStorage,
/// };
/// # use im::Vector;
/// # use std::path::Path;
/// # use hotg_rune_compiler_2::{
/// # proc_blocks::{ProcBlockRegistry, LoadError}, FileSystem, FileSystemError,
/// # };
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
///     // ...
/// #   fn read(&self, path: &Path) -> Result<Vector<u8>, FileSystemError> { todo!() }
/// }
/// impl ProcBlockRegistry for Database {
///     // ...
/// #   fn load_proc_block_binary(
/// #       &self,
/// #       path: uriparse::URI<'_>,
/// #   ) -> Result<Vector<u8>, LoadError> { todo!() }
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
pub trait Frontend: Environment + FileSystem + ProcBlockRegistry {
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
            Arc::new(NoSuchResource { name }) as Arc<dyn Error>
        })?;

    match (inline, path) {
        (Some(inline), None) => Ok(inline.as_bytes().into()),
        (None, Some(path)) => db
            .read(path.as_ref())
            .map_err(|e| Arc::new(e) as Arc<dyn Error>),
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
        if let Stage::ProcBlock(ProcBlockStage { proc_block, .. }) = stage {
            let proc_block = proc_block
                .to_uri()
                .map_err(|e| Arc::new(e) as Arc<dyn Error>)?;

            let binary = db
                .load_proc_block_binary(proc_block)
                .map_err(|e| Arc::new(e) as Arc<dyn Error>)?;

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
        .ok_or_else(|| NoSuchProcBlock { name })
        .map_err(|e| Arc::new(e) as Arc<dyn Error>)?;

    if let Stage::ProcBlock(ProcBlockStage { proc_block, .. }) = stage {
        let proc_block = proc_block
            .to_uri()
            .map_err(|e| Arc::new(e) as Arc<dyn Error>)?;

        db.load_proc_block_binary(proc_block)
            .map_err(|e| Arc::new(e) as Arc<dyn Error>)
    } else {
        todo!()
    }
}

#[tracing::instrument(skip(db), err)]
fn resource_values(
    db: &dyn Frontend,
) -> Result<OrdMap<Text, Vector<u8>>, Arc<dyn Error>> {
    let doc = db.parse()?;

    let mut proc_blocks = OrdMap::new();

    for (name, stage) in &doc.pipeline {
        if let Stage::ProcBlock(ProcBlockStage { proc_block, .. }) = stage {
            let proc_block = proc_block
                .to_uri()
                .map_err(|e| Arc::new(e) as Arc<dyn Error>)?;

            let binary = db
                .load_proc_block_binary(proc_block)
                .map_err(|e| Arc::new(e) as Arc<dyn Error>)?;

            proc_blocks.insert(Text::from(name), binary);
        }
    }

    Ok(proc_blocks)
}

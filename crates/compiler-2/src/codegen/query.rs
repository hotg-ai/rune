use std::sync::Arc;

use im::{OrdMap, Vector};

use crate::{
    codegen::runefile,
    config::Environment,
    lowering::{self, HirDB, HirId, ResourceId},
    parse, FileSystem, Text,
};

#[salsa::query_group(CodegenStorage)]
pub trait Codegen: HirDB + Environment + FileSystem {
    /// Generate a `Runefile.yml` for a self-contained Rune (i.e. saving all
    /// files locally).
    ///
    /// Note that this *isn't* just a case of taking the original `Runefile.yml`
    /// and patching some paths. Instead, we generate the entire
    /// [`parse::Document`] based on the HIR in our [`HirDB`].
    #[salsa::dependencies]
    #[salsa::invoke(crate::codegen::runefile::runefile)]
    fn self_contained_runefile(&self) -> Arc<parse::Document>;

    fn resource_files(&self) -> OrdMap<Text, Vector<u8>>;
}

#[tracing::instrument(skip(db))]
fn resource_files(db: &dyn Codegen) -> OrdMap<Text, Vector<u8>> {
    let mut files = OrdMap::new();

    for (name, id) in resource_names(db) {
        let lowering::Resource { default_value, .. } = db.lookup_resource(id);
        let filename = Text::new(runefile::resource_file_name(&name));

        match default_value {
            Some(lowering::ResourceSource::FromDisk { filename: _ }) => {
                todo!();
            },
            Some(lowering::ResourceSource::Inline(inline)) => {
                files.insert(filename, inline.as_bytes().into());
            },
            None => {},
        }
    }

    files
}

pub(crate) fn resource_names(
    db: &dyn Codegen,
) -> impl Iterator<Item = (Text, ResourceId)> {
    let (names, _) = db.names();
    names.into_iter().filter_map(|(name, id)| match id {
        HirId::Resource(id) => Some((name, id)),
        _ => None,
    })
}

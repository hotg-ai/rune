use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use anyhow::{Context, Error};
use hotg_rune_syntax::hir::ModelFile;
use crate::Environment;

pub(crate) fn load(
    rune: &hotg_rune_syntax::hir::Rune,
    env: &mut dyn Environment,
) -> Result<HashMap<PathBuf, Vec<u8>>, Error> {
    let mut models = HashMap::new();

    for (id, model) in rune.models() {
        if let ModelFile::FromDisk(filename) = &model.model_file {
            let name = rune.names.get_name(id)
                .with_context(|| format!("Unable to get the name of the model using \"{}\", this may be a rune bug", filename.display()))?;

            let raw = env
                .read_file(filename)
                .with_context(|| format!("Unable to load {}'s model", name))?;
            let path = Path::new("models").join(filename.file_name().unwrap());
            models.insert(path, raw);
        }
    }

    Ok(models)
}

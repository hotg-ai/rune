use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use anyhow::{Context, Error};
use hotg_rune_syntax::hir::{ModelFile, ResourceSource};
use crate::Environment;

pub(crate) fn load(
    rune: &hotg_rune_syntax::hir::Rune,
    env: &mut dyn Environment,
) -> Result<HashMap<PathBuf, Vec<u8>>, Error> {
    let mut files = HashMap::new();

    load_models(rune, env, &mut files)?;
    load_file_resources(rune, env, &mut files)?;

    Ok(files)
}

fn load_models(
    rune: &hotg_rune_syntax::hir::Rune,
    env: &mut dyn Environment,
    files: &mut HashMap<PathBuf, Vec<u8>>,
) -> Result<(), Error> {
    for (id, model) in rune.models() {
        if let ModelFile::FromDisk(filename) = &model.model_file {
            let name = rune.get_name_by_id(id)
                .with_context(|| format!("Unable to get the name of the model using \"{}\", this may be a rune bug", filename.display()))?;

            let raw = env
                .read_file(filename)
                .with_context(|| format!("Unable to load {}'s model", name))?;
            let path = Path::new("models").join(filename.file_name().unwrap());
            files.insert(path, raw);
        }
    }

    Ok(())
}

fn load_file_resources(
    rune: &hotg_rune_syntax::hir::Rune,
    env: &mut dyn Environment,
    files: &mut HashMap<PathBuf, Vec<u8>>,
) -> Result<(), Error> {
    for (id, resource) in &rune.resources {
        if let Some(ResourceSource::FromDisk(filename)) = &resource.source {
            let name = rune.get_name_by_id(*id)
                .with_context(|| format!("Unable to get the name of the model using \"{}\", this may be a rune bug", filename.display()))?;

            let raw = env.read_file(filename).with_context(|| {
                format!("Unable to the \"{}\" resource", name)
            })?;
            let path =
                Path::new("resources").join(filename.file_name().unwrap());
            files.insert(path, raw);
        }
    }

    Ok(())
}

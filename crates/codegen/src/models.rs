use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use anyhow::{Context, Error};
use crate::Environment;

pub(crate) fn load(
    rune: &rune_syntax::hir::Rune,
    env: &mut dyn Environment,
) -> Result<HashMap<PathBuf, Vec<u8>>, Error> {
    let mut models = HashMap::new();

    for (id, model) in rune.models() {
        let name = rune.names.get_name(id)
            .with_context(|| format!("Unable to get the name of the model using \"{}\", this may be a rune bug", model.model_file.display()))?;

        let raw = env
            .read_file(&model.model_file)
            .with_context(|| format!("Unable to load {}'s model", name))?;
        let path = Path::new(name).with_extension("tflite");
        models.insert(path, raw);
    }

    Ok(models)
}

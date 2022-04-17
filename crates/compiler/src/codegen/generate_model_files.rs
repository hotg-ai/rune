use std::path::Path;

use legion::systems::CommandBuffer;

use crate::{
    codegen::File,
    lowering::{ModelData, Name},
};

/// Create a [`File`] for each model with associated [`ModelData`] and put it in
/// the `models/` directory.
#[legion::system(for_each)]
pub(crate) fn run(cmd: &mut CommandBuffer, name: &Name, data: &ModelData) {
    let path = Path::new("models").join(name.as_str());
    let file = File::new(path, data.0.clone());
    cmd.push((file,));
}

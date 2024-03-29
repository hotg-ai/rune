use codespan::Span;
use legion::{systems::CommandBuffer, Entity};

use crate::{
    lowering::{Model, ModelData, ModelFile, Name},
    BuildContext, Diagnostics,
};

#[legion::system(for_each)]
pub(crate) fn run(
    cmd: &mut CommandBuffer,
    #[resource] diags: &mut Diagnostics,
    #[resource] build_ctx: &BuildContext,
    &entity: &Entity,
    name: &Name,
    model: &Model,
    &span: &Span,
) {
    match &model.model_file {
        ModelFile::FromDisk(path) => {
            match super::load_resource_data::load(
                &build_ctx.current_directory,
                path,
                name,
                span,
            ) {
                Ok(data) => cmd.add_component(entity, ModelData::from(data)),
                Err(diag) => diags.push(diag),
            }
        },
        ModelFile::Resource(_) => {},
    }
}

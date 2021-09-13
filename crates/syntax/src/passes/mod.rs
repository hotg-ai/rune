mod check_for_loops;
mod construct_pipeline;
mod context;
mod register_output_slots;
mod register_resources;
mod register_stages;

pub(crate) use self::context::Context;

use crate::{Diagnostics, hir::Rune, yaml::*};

pub fn analyse(doc: &Document, diags: &mut Diagnostics) -> Rune {
    let mut ctx = Context::new(diags);

    match doc {
        Document::V1 {
            image,
            pipeline,
            resources,
        } => {
            ctx.rune.base_image = Some(image.clone().into());

            register_resources::run(&mut ctx, resources);
            register_stages::run(&mut ctx, pipeline);
            register_output_slots::run(&mut ctx, pipeline);
            construct_pipeline::run(&mut ctx, pipeline);
            check_for_loops::run(&mut ctx);
        },
    }

    ctx.rune
}

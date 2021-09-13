use codespan_reporting::diagnostic::{Diagnostic, Label};
use indexmap::IndexMap;

use crate::{
    hir::{self, Node},
    passes::Context,
    yaml::Stage,
    utils::range_span,
};

pub(crate) fn run(ctx: &mut Context<'_>, pipeline: &IndexMap<String, Stage>) {
    for (name, stage) in pipeline {
        let span = stage.span();

        match hir::Stage::from_yaml(
            stage.clone(),
            |id| ctx.rune.get_resource(&id).is_some(),
            |name| ctx.rune.get_id_by_name(name),
        ) {
            Ok(stage) => {
                let id = ctx.ids.next();
                ctx.rune.register_stage(
                    id,
                    Node {
                        stage,
                        input_slots: Vec::new(),
                        output_slots: Vec::new(),
                    },
                );
                ctx.register_name(name, id, span);
            },
            Err(e) => {
                let diag = Diagnostic::error()
                    .with_message(e.to_string())
                    .with_labels(vec![Label::primary((), range_span(span))]);
                ctx.diags.push(diag);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Diagnostics,
        utils::dummy_document,
        yaml::{Document, DocumentV1},
    };
    use super::*;

    #[test]
    fn register_all_stages() {
        let pipeline = match dummy_document() {
            Document::V1(DocumentV1 { pipeline, .. }) => pipeline,
        };
        let mut diags = Diagnostics::new();
        let mut ctx = Context::new(&mut diags);
        let stages = vec!["audio", "fft", "model", "label", "output"];

        run(&mut ctx, &pipeline);

        for stage_name in stages {
            let id = ctx.rune.get_id_by_name(stage_name).unwrap();
            assert!(ctx.rune.get_stage(&id).is_some());
        }

        assert!(diags.is_empty());
    }
}

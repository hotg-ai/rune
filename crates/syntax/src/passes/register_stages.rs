use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use indexmap::IndexMap;

use crate::{
    Diagnostics,
    hir::{self, HirId, NameTable, Node, Resource},
    passes::helpers,
    utils::{HirIds, range_span},
    yaml::Stage,
};

pub(crate) fn run(
    ids: &mut HirIds,
    pipeline: &IndexMap<String, Stage>,
    spans: &IndexMap<HirId, Span>,
    stages: &mut IndexMap<HirId, Node>,
    resources: &IndexMap<HirId, Resource>,
    names: &mut NameTable,
    diags: &mut Diagnostics,
) {
    for (name, stage) in pipeline {
        let span = stage.span();

        match hir::Stage::from_yaml(
            stage.clone(),
            |id| resources.contains_key(&id),
            |name| names.get_id(name),
        ) {
            Ok(stage) => {
                let id = ids.next();
                stages.insert(
                    id,
                    Node {
                        stage,
                        input_slots: Vec::new(),
                        output_slots: Vec::new(),
                    },
                );
                helpers::register_name(name, id, span, &spans, names, diags);
            },
            Err(e) => {
                let diag = Diagnostic::error()
                    .with_message(e.to_string())
                    .with_labels(vec![Label::primary((), range_span(span))]);
                diags.push(diag);
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
        let stage_names = vec!["audio", "fft", "model", "label", "output"];
        let mut ids = HirIds::new();
        let spans = IndexMap::default();
        let resources = IndexMap::default();
        let mut stages = IndexMap::default();
        let mut names = NameTable::default();

        run(
            &mut ids,
            &pipeline,
            &spans,
            &mut stages,
            &resources,
            &mut names,
            &mut diags,
        );

        for stage_name in stage_names {
            let id = names.get_id(stage_name).unwrap();
            assert!(stages.get(&id).is_some());
        }

        assert!(diags.is_empty());
    }
}

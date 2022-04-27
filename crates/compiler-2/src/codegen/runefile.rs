use std::sync::Arc;

use indexmap::IndexMap;

use crate::{
    codegen::{query, Codegen},
    lowering::{self, HirId, NodeId, NodeKind},
    parse, Text,
};

#[tracing::instrument(skip(db))]
pub(crate) fn runefile(db: &dyn Codegen) -> Arc<parse::Document> {
    let doc = parse::DocumentV1 {
        version: 1,
        image: parse::Image::runicos_base(),
        pipeline: pipeline(db),
        resources: resources(db),
    };

    Arc::new(parse::Document::V1(doc))
}

#[tracing::instrument(skip(db))]
fn pipeline(db: &dyn Codegen) -> IndexMap<String, parse::Stage> {
    let mut stages = IndexMap::new();

    for (name, id) in node_names(db) {
        let node = db.lookup_node(id);

        let (inputs, _) = db.inputs(id);
        let inputs: Vec<_> = inputs
            .into_iter()
            .filter_map(|i| i)
            .map(|lowering::Input { index, node }| {
                let (input_name, _) =
                    node_names(db).find(|(_, id)| *id == node).expect(
                        "You can't have a NodeId without a corresponding name",
                    );

                crate::parse::Input {
                    index: Some(index),
                    name: input_name.to_string(),
                }
            })
            .collect();

        let outputs: Vec<_> = node.outputs.into_iter().collect();

        let args = arguments(db, id);

        let stage = match node.kind {
            NodeKind::Input => {
                parse::Stage::Capability(parse::CapabilityStage {
                    capability: resource_or_text(db, node.identifier)
                        .to_string(),
                    args,
                    outputs,
                })
            },
            NodeKind::ProcBlock => {
                parse::Stage::ProcBlock(parse::ProcBlockStage {
                    proc_block: resource_or_text(db, node.identifier)
                        .to_string()
                        .parse()
                        .unwrap(),
                    outputs,
                    args,
                    inputs,
                })
            },
            NodeKind::Model => parse::Stage::Model(parse::ModelStage {
                model: resource_or_text(db, node.identifier),
                outputs,
                args,
                inputs,
            }),
            NodeKind::Output => parse::Stage::Out(parse::OutStage {
                out: resource_or_text(db, node.identifier).to_string(),
                args,
                inputs,
            }),
        };

        stages.insert(name.to_string(), stage);
    }

    stages
}

fn resource_or_text(
    db: &dyn Codegen,
    value: lowering::ResourceOrText,
) -> parse::ResourceOrString {
    match value {
        lowering::ResourceOrText::Text(text) => {
            parse::ResourceOrString::String(text.to_string())
        },
        lowering::ResourceOrText::Resource(resource) => {
            let (name, _) = query::resource_names(db)
                .find(|(_, id)| *id == resource)
                .expect(
                    "You can't have a ResourceId without a corresponding name",
                );

            parse::ResourceOrString::Resource(parse::ResourceName(
                name.to_string(),
            ))
        },
        lowering::ResourceOrText::Error => todo!(),
    }
}

fn arguments(
    db: &dyn Codegen,
    node_id: NodeId,
) -> IndexMap<String, parse::Argument> {
    let (args, _) = db.arguments(node_id);

    args.into_iter()
        .map(|(name, id)| {
            let arg = db.lookup_argument(id);
            (
                name.to_string(),
                parse::Argument(resource_or_text(db, arg.value)),
            )
        })
        .collect()
}

#[tracing::instrument(skip(db))]
fn resources(db: &dyn Codegen) -> IndexMap<String, parse::ResourceDeclaration> {
    let mut resources = IndexMap::new();

    for (name, id) in query::resource_names(db) {
        let lowering::Resource { default_value, ty } = db.lookup_resource(id);

        resources.insert(
            name.to_string(),
            parse::ResourceDeclaration {
                path: default_value.map(|_| resource_file_name(&name)),
                inline: None,
                ty,
            },
        );
    }

    resources
}

pub(crate) fn resource_file_name(node_name: &str) -> String {
    format!("resources/{}.bin", node_name)
}

fn node_names(db: &dyn Codegen) -> impl Iterator<Item = (Text, NodeId)> {
    let (names, _) = db.names();
    names.into_iter().filter_map(|(name, id)| match id {
        HirId::Node(id) => Some((name, id)),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use im::Vector;

    use super::*;
    use crate::{
        codegen::CodegenStorage, lowering::HirDBStorage, EnvironmentStorage,
    };

    #[derive(Default)]
    #[salsa::database(HirDBStorage, CodegenStorage, EnvironmentStorage)]
    struct DB {
        storage: salsa::Storage<Self>,
    }

    impl salsa::Database for DB {}

    impl crate::FileSystem for DB {
        fn read(
            &self,
            _path: &std::path::Path,
        ) -> Result<Vector<u8>, crate::FileSystemError> {
            todo!();
        }
    }

    #[test]
    fn round_trip_a_runefile() {
        let src = r#"
            version: 1
            image: "runicos/base"
            pipeline:
                image:
                    capability: IMAGE
                    outputs:
                        - type: f32
                          dimensions: [1]
                modulo:
                    proc-block: ./modulo
                    inputs:
                        - image.0
                    outputs:
                        - type: f32
                          dimensions: [1]
                sine:
                    model: sine.tflite
                    inputs:
                        - modulo.0
                    outputs:
                        - type: f32
                          dimensions: [1]
                output:
                    out: SERIAL
                    inputs:
                        - sine.0

            resources:
                res: {}
        "#;
        let mut db = DB::default();
        let doc = crate::parse::parse_runefile(src).unwrap();
        crate::lowering::populate_from_document(&mut db, doc.clone());

        let runefile = Arc::try_unwrap(db.self_contained_runefile()).unwrap();

        assert_eq!(runefile, doc);
    }
}

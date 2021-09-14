use codespan_reporting::diagnostic::{Diagnostic, Label};
use legion::{Entity, Query, systems::CommandBuffer, world::SubWorld};
use crate::{
    Diagnostics,
    hir::{Model, ModelFile, NameTable, ProcBlock, Resource, Sink, Source},
    yaml::{self, DocumentV1, ResourceName, ResourceOrString, ResourceType},
};

/// Attach [`Model`], [`ProcBlock`], [`Sink`], and [`Source`] components to
/// each [`yaml::Stage`] in the [`DocumentV1`].
#[legion::system]
#[read_component(Resource)]
pub(crate) fn run(
    cmd: &mut CommandBuffer,
    world: &SubWorld,
    #[resource] doc: &DocumentV1,
    #[resource] names: &NameTable,
    #[resource] diags: &mut Diagnostics,
    query: &mut Query<&Resource>,
) {
    for (name, stage) in &doc.pipeline {
        let ent = match names.get(name) {
            Some(&e) => e,
            None => continue,
        };

        match stage {
            yaml::Stage::Model {
                model: ResourceOrString::Resource(r),
                ..
            } => {
                match resource_model(r, names, |ent| query.get(world, ent).ok())
                {
                    Ok(model) => cmd.add_component(ent, model),
                    Err(diag) => diags.push(diag),
                }
            },
            yaml::Stage::Model {
                model: ResourceOrString::String(path),
                ..
            } => cmd.add_component(
                ent,
                Model {
                    model_file: ModelFile::FromDisk(path.into()),
                },
            ),
            yaml::Stage::ProcBlock {
                proc_block, args, ..
            } => cmd.add_component(
                ent,
                ProcBlock {
                    path: proc_block.clone(),
                    parameters: args.clone().into_iter().collect(),
                },
            ),
            yaml::Stage::Capability {
                capability, args, ..
            } => cmd.add_component(
                ent,
                Source {
                    kind: capability.as_str().into(),
                    parameters: args.clone().into_iter().collect(),
                },
            ),
            yaml::Stage::Out { out, .. } => cmd.add_component(
                ent,
                Sink {
                    kind: out.as_str().into(),
                },
            ),
        }
    }
}

fn resource_model<'a>(
    resource_name: &yaml::ResourceName,
    names: &NameTable,
    get_resource: impl FnOnce(Entity) -> Option<&'a Resource> + 'a,
) -> Result<Model, Diagnostic<()>> {
    let ent = match names.get(resource_name.as_str()) {
        Some(&e) => e,
        None => return Err(unknown_resource_diagnostic(resource_name)),
    };

    let res = match get_resource(ent) {
        Some(r) => r,
        None => return Err(not_a_resource_diagnostic(resource_name)),
    };

    if res.ty != ResourceType::Binary {
        return Err(model_resource_should_be_binary_diagnostic(resource_name));
    }

    Ok(Model {
        model_file: ModelFile::Resource(ent),
    })
}

fn model_resource_should_be_binary_diagnostic(
    resource_name: &ResourceName,
) -> Diagnostic<()> {
    Diagnostic::error()
        .with_message(format!(
            "\"{}\" should be a binary resource",
            resource_name
        ))
        .with_labels(vec![Label::primary((), resource_name.span())])
}

fn not_a_resource_diagnostic(resource_name: &ResourceName) -> Diagnostic<()> {
    Diagnostic::error()
        .with_message(format!("\"{}\" is not a resource", resource_name))
        .with_labels(vec![Label::primary((), resource_name.span())])
}

fn unknown_resource_diagnostic(resource_name: &ResourceName) -> Diagnostic<()> {
    Diagnostic::error()
        .with_message(format!("No definition for \"{}\"", resource_name))
        .with_labels(vec![Label::primary((), resource_name.span())])
}

#[cfg(test)]
mod tests {
    use legion::{World, IntoQuery};

    use crate::{
        BuildContext,
        hir::{Name, SinkKind, SourceKind},
        passes::{self, Schedule},
        yaml::{ResourceDeclaration, ResourceType, Stage, Value},
    };
    use super::*;

    fn doc() -> DocumentV1 {
        DocumentV1 {
            image: "img".parse().unwrap(),
            pipeline: map! {
                cap: Stage::Capability {
                    capability: "SOUND".to_string(),
                    args: map! {
                        hz: Value::from(128),
                    },
                    outputs: Vec::new(),
                },
                transform: Stage::ProcBlock {
                    proc_block: "my-proc-block".parse().unwrap(),
                    args: map! {
                        some_arg: Value::from("asdf"),
                    },
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                },
                model_from_disk: Stage::Model {
                    model: ResourceOrString::String("model.tflite".into()),
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                },
                model_from_resource: Stage::Model {
                    model: ResourceOrString::Resource("$MODEL_FILE".parse().unwrap()),
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                },
                model_with_not_a_resource: Stage::Model {
                    model: ResourceOrString::Resource("$cap".parse().unwrap()),
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                },
                model_with_missing_resource: Stage::Model {
                    model: ResourceOrString::Resource("$NON_EXISTENT".parse().unwrap()),
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                },
                model_with_string_resource: Stage::Model {
                    model: ResourceOrString::Resource("$STRING_RESOURCE".parse().unwrap()),
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                },
                serial: Stage::Out {
                    out: "SERIAL".to_string(),
                    args: Default::default(),
                    inputs: Vec::new(),
                },
            },
            resources: map! {
                MODEL_FILE: ResourceDeclaration {
                    inline: None,
                    path: Some("model.tflite".to_string()),
                    ty: ResourceType::Binary,
                },
                STRING_RESOURCE: ResourceDeclaration {
                    inline: Some("res".to_string()),
                    path: None,
                    ty: ResourceType::String,
                },
            },
        }
    }

    #[test]
    fn register_all_stages() {
        let mut world = World::default();
        let mut res =
            passes::initialize_resources(BuildContext::from_doc(doc().into()));

        Schedule::new()
            .and_then(passes::parse::run_system())
            .and_then(passes::register_names::run_system())
            .and_then(passes::update_nametable::run_system())
            .and_then(passes::register_resources::run_system())
            .and_then(run_system())
            .run(&mut world, &mut res);

        let diags = res.get::<Diagnostics>().unwrap();
        let diags: Vec<_> = diags.iter().collect();
        assert_eq!(diags.len(), 3);
        assert_eq!(diags[0].message, "\"$cap\" is not a resource");
        assert_eq!(diags[1].message, "No definition for \"$NON_EXISTENT\"");
        assert_eq!(
            diags[2].message,
            "\"$STRING_RESOURCE\" should be a binary resource"
        );

        let proc_blocks_should_be = vec![(
            Name::from("transform"),
            ProcBlock {
                path: "my-proc-block".parse().unwrap(),
                parameters: map! {
                    some_arg: Value::from("asdf"),
                },
            },
        )];
        let got: Vec<_> = <(&Name, &ProcBlock)>::query()
            .iter(&world)
            .map(|(n, p)| (n.clone(), p.clone()))
            .collect();
        assert_eq!(got, proc_blocks_should_be);

        let models_should_be = vec![
            (
                Name::from("model_from_disk"),
                Model {
                    model_file: ModelFile::FromDisk("model.tflite".into()),
                },
            ),
            (
                Name::from("model_from_resource"),
                Model {
                    model_file: ModelFile::Resource(
                        *res.get::<NameTable>()
                            .unwrap()
                            .get("MODEL_FILE")
                            .unwrap(),
                    ),
                },
            ),
        ];
        let got: Vec<_> = <(&Name, &Model)>::query()
            .iter(&world)
            .map(|(n, m)| (n.clone(), m.clone()))
            .collect();
        assert_eq!(got, models_should_be);

        let sources_should_be = vec![(
            Name::from("cap"),
            Source {
                kind: SourceKind::Sound,
                parameters: map! {
                    hz: Value::from(128),
                },
            },
        )];
        let got: Vec<_> = <(&Name, &Source)>::query()
            .iter(&world)
            .map(|(n, s)| (n.clone(), s.clone()))
            .collect();
        assert_eq!(got, sources_should_be);

        let sinks_should_be = vec![(
            Name::from("serial"),
            Sink {
                kind: SinkKind::Serial,
            },
        )];
        let got: Vec<_> = <(&Name, &Sink)>::query()
            .iter(&world)
            .map(|(n, s)| (n.clone(), s.clone()))
            .collect();
        assert_eq!(got, sinks_should_be);
    }
}

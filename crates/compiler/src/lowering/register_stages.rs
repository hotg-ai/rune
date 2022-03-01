use codespan_reporting::diagnostic::{Diagnostic, Label};
use indexmap::IndexMap;
use legion::{systems::CommandBuffer, world::SubWorld, Entity, Query};

use crate::{
    lowering::{
        self, Mimetype, Model, ModelFile, NameTable, ProcBlock, Resource,
        ResourceData, Sink, Source,
    },
    parse::{
        self, CapabilityStage, DocumentV1, ModelStage, OutStage,
        ProcBlockStage, ResourceName, ResourceType,
    },
    Diagnostics,
};

/// Attach [`Model`], [`ProcBlock`], [`Sink`], and [`Source`] components to
/// each [`parse::Stage`] in the [`DocumentV1`].
#[legion::system]
#[read_component(Resource)]
pub(crate) fn run(
    cmd: &mut CommandBuffer,
    world: &SubWorld,
    #[resource] doc: &DocumentV1,
    #[resource] names: &NameTable,
    #[resource] diags: &mut Diagnostics,
    resources: &mut Query<(&Resource, Option<&ResourceData>)>,
) {
    for (name, stage) in &doc.pipeline {
        let ent = match names.get(name) {
            Some(&e) => e,
            None => continue,
        };

        let args = match translate_args(stage.args(), names) {
            Ok(a) => a,
            Err(diag) => {
                diags.push(diag);
                continue;
            },
        };

        match stage {
            parse::Stage::Model(ModelStage { model, .. }) => {
                match register_model(names, name, model, &args, |e: Entity| {
                    resources.get(world, e).ok()
                }) {
                    Ok((model, mimetype)) => {
                        cmd.add_component(ent, model);
                        cmd.add_component(ent, mimetype);
                    },
                    Err(diag) => diags.push(diag),
                }
            },
            parse::Stage::ProcBlock(ProcBlockStage { proc_block, .. }) => {
                if proc_block.version.is_none() {
                    let diag = warn_on_unversioned_proc_block_diagnostic(
                        name, proc_block,
                    );
                    diags.push(diag);
                }

                cmd.add_component(
                    ent,
                    ProcBlock {
                        path: proc_block.clone(),
                        parameters: args,
                    },
                )
            },
            parse::Stage::Capability(CapabilityStage {
                capability, ..
            }) => cmd.add_component(
                ent,
                Source {
                    kind: capability.as_str().into(),
                    parameters: args,
                },
            ),
            parse::Stage::Out(OutStage { out, .. }) => cmd.add_component(
                ent,
                Sink {
                    kind: out.as_str().into(),
                    args,
                },
            ),
        }
    }
}

fn warn_on_unversioned_proc_block_diagnostic(
    name: &str,
    proc_block: &parse::Path,
) -> Diagnostic<()> {
    let msg = format!(
        "The \"{}\" proc block used by \"{}\" should have a version specifier",
        proc_block, name
    );
    let versioned = parse::Path {
        version: Some(env!("CARGO_PKG_VERSION").to_string()),
        ..proc_block.clone()
    };

    Diagnostic::warning()
        .with_message(msg)
        .with_notes(vec![format!(
            "hint: change it to something like \"{}\"",
            versioned
        )])
}

fn translate_args(
    args: &IndexMap<String, parse::Argument>,
    names: &NameTable,
) -> Result<IndexMap<String, lowering::ResourceOrString>, Diagnostic<()>> {
    let mut translated = IndexMap::new();

    for (name, value) in args {
        let value = match &value.0 {
            parse::ResourceOrString::Resource(r) => match names
                .get(r.as_str())
                .copied()
            {
                Some(entity) => lowering::ResourceOrString::Resource(entity),
                None => return Err(not_a_resource_diagnostic(r)),
            },
            parse::ResourceOrString::String(s) => {
                lowering::ResourceOrString::String(s.clone())
            },
        };

        translated.insert(name.clone(), value);
    }

    Ok(translated)
}

fn register_model<'a>(
    names: &NameTable,
    node_name: &str,
    model: &parse::ResourceOrString,
    args: &IndexMap<String, lowering::ResourceOrString>,
    mut get_resource: impl FnMut(Entity) -> Option<(&'a Resource, Option<&'a ResourceData>)>
        + 'a,
) -> Result<(Model, Mimetype), Diagnostic<()>> {
    let (mimetype, args) = model_format_and_args(node_name, args, |e| {
        get_resource(e).and_then(|r| r.1).cloned()
    })?;

    let model_file = match model {
        parse::ResourceOrString::Resource(resource_name) => {
            resource_model(resource_name, names, |e| {
                get_resource(e).map(|r| r.0)
            })?
        },
        parse::ResourceOrString::String(s) => ModelFile::FromDisk(s.into()),
    };

    Ok((Model { model_file, args }, mimetype))
}

fn model_format_and_args(
    node_name: &str,
    args: &IndexMap<String, lowering::ResourceOrString>,
    get_resource_data: impl FnOnce(Entity) -> Option<ResourceData>,
) -> Result<
    (Mimetype, IndexMap<String, lowering::ResourceOrString>),
    Diagnostic<()>,
> {
    let mut args = args.clone();

    let mimetype = match args.remove("format") {
        Some(lowering::ResourceOrString::String(format)) => {
            mimetype_for_known_format(&format)?
        },
        Some(lowering::ResourceOrString::Resource(entity)) => {
            match get_resource_data(entity) {
                Some(data) => match std::str::from_utf8(&data) {
                    Ok(format) => mimetype_for_known_format(format)?,
                    Err(e) => {
                        return Err(invalid_mimetype_diagnostic(node_name, e))
                    },
                },
                None => {
                    todo!("Handle unknown resource in the format")
                },
            }
        },
        None => Mimetype::default(),
    };

    Ok((mimetype, args))
}

fn invalid_mimetype_diagnostic(
    node_name: &str,
    e: std::str::Utf8Error,
) -> Diagnostic<()> {
    let msg = format!("Invalid format for \"{}\": {}", node_name, e);

    Diagnostic::error().with_message(msg)
}

fn mimetype_for_known_format(format: &str) -> Result<Mimetype, Diagnostic<()>> {
    let known_formats = [
        ("onnx", hotg_rune_core::ONNX_MIMETYPE),
        ("tensorflow", hotg_rune_core::TF_MIMETYPE),
        ("tensorflow-js", hotg_rune_core::TFJS_MIMETYPE),
        ("tensorflow-lite", hotg_rune_core::TFLITE_MIMETYPE),
    ];

    known_formats
        .iter()
        .find(|(name, _)| *name == format)
        .map(|(_, mt)| Mimetype::from(*mt))
        .ok_or_else(|| {
            unknown_format_diagnostic(
                &format,
                known_formats.iter().copied().map(|(f, _)| f),
            )
        })
}

fn unknown_format_diagnostic(
    format: &str,
    expected: impl Iterator<Item = &'static str>,
) -> Diagnostic<()> {
    // TODO: use span information to tell the user where the error came from

    let msg = format!(
        "Expected the format to be one of {}, but found {:?}",
        join(expected, ", "),
        format
    );
    Diagnostic::error().with_message(msg)
}

fn join<'a>(items: impl Iterator<Item = &'a str>, separator: &str) -> String {
    let mut buffer = String::new();

    for (i, item) in items.enumerate() {
        if i > 0 {
            buffer.push_str(separator);
        }

        buffer.push_str(item);
    }

    buffer
}

fn resource_model<'a>(
    resource_name: &parse::ResourceName,
    names: &NameTable,
    get_resource: impl FnOnce(Entity) -> Option<&'a Resource> + 'a,
) -> Result<ModelFile, Diagnostic<()>> {
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

    Ok(ModelFile::Resource(ent))
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
    use indexmap::IndexMap;
    use legion::{IntoQuery, Resources, World};

    use super::*;
    use crate::{
        lowering::{self, Name, SinkKind, SourceKind},
        parse::{ResourceDeclaration, ResourceType, Stage},
        phases::Phase,
        BuildContext,
    };

    fn doc() -> DocumentV1 {
        DocumentV1 {
            version: 1,
            image: "img".parse().unwrap(),
            pipeline: map! {
                cap: Stage::Capability(CapabilityStage {
                    capability: "SOUND".to_string(),
                    args: map! {
                        hz: "128".into(),
                    },
                    outputs: Vec::new(),
                }),
                transform: Stage::ProcBlock(ProcBlockStage {
                    proc_block: "my-proc-block".parse().unwrap(),
                    args: map! {
                        some_arg: "asdf".into(),
                    },
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                }),
                model_from_disk: Stage::Model(ModelStage {
                    model: parse::ResourceOrString::String("model.tflite".into()),
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                    args: IndexMap::new(),
                }),
                model_from_resource: Stage::Model(ModelStage {
                    model: parse::ResourceOrString::Resource("$MODEL_FILE".parse().unwrap()),
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                    args: IndexMap::new(),
                }),
                model_with_not_a_resource: Stage::Model(ModelStage {
                    model: parse::ResourceOrString::Resource("$cap".parse().unwrap()),
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                    args: IndexMap::new(),
                }),
                model_with_missing_resource: Stage::Model(ModelStage {
                    model: parse::ResourceOrString::Resource("$NON_EXISTENT".parse().unwrap()),
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                    args: IndexMap::new(),
                }),
                model_with_string_resource: Stage::Model(ModelStage {
                    model: parse::ResourceOrString::Resource("$STRING_RESOURCE".parse().unwrap()),
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                    args: IndexMap::new(),
                }),
                serial: Stage::Out(OutStage {
                    out: "SERIAL".to_string(),
                    args: Default::default(),
                    inputs: Vec::new(),
                }),
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
        let mut res = Resources::default();
        res.insert(BuildContext::from_doc(doc().into()));
        res.insert(NameTable::default());
        crate::parse::phase().run(&mut world, &mut res);

        Phase::new()
            .and_then(lowering::register_names::run_system)
            .and_then(lowering::update_nametable::run_system)
            .and_then(lowering::register_resources::run_system)
            .and_then(run_system)
            .run(&mut world, &mut res);

        let diags = res.get::<Diagnostics>().unwrap();
        let diags: Vec<_> = diags.iter().collect();
        assert_eq!(diags.len(), 4);
        assert_eq!(
            diags[0],
            &Diagnostic::warning()
                .with_message(
                    "The \"my-proc-block\" proc block used by \"transform\" \
                     should have a version specifier"
                )
                .with_notes(vec![format!(
                    "hint: change it to something like \"my-proc-block@{}\"",
                    env!("CARGO_PKG_VERSION").to_string()
                )
                .to_string()])
        );
        assert_eq!(diags[1].message, "\"$cap\" is not a resource");
        assert_eq!(diags[2].message, "No definition for \"$NON_EXISTENT\"");
        assert_eq!(
            diags[3].message,
            "\"$STRING_RESOURCE\" should be a binary resource"
        );

        let proc_blocks_should_be = vec![(
            Name::from("transform"),
            ProcBlock {
                path: "my-proc-block".parse().unwrap(),
                parameters: map! {
                    some_arg: "asdf".into(),
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
                    args: IndexMap::new(),
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
                    args: IndexMap::new(),
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
                    hz: "128".into(),
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
                args: map! {},
            },
        )];
        let got: Vec<_> = <(&Name, &Sink)>::query()
            .iter(&world)
            .map(|(n, s)| (n.clone(), s.clone()))
            .collect();
        assert_eq!(got, sinks_should_be);
    }
}

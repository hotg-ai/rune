use im::{OrdMap, Vector};
use indexmap::IndexMap;

use crate::{
    diagnostics::{AsDiagnostic, Diagnostics},
    lowering::{
        Abi, Argument, ArgumentId, DuplicateName, HirId, Input, Node, NodeId,
        NodeKind, NotAResource, PathAndInlineNotAllowed, Resource, ResourceId,
        ResourceOrText, ResourceSource, ResourceUsedAsInput, UnknownAbi,
        UnknownInput, UnknownResource,
    },
    parse, Text,
};

/// Populate a [`HirDB`] using a [`parse::Document`].
#[tracing::instrument(skip(db, doc))]
pub fn populate_from_document(db: &mut dyn HirDB, doc: parse::Document) {
    let parse::DocumentV1 {
        // Only used to switch between Runefile.yml formats
        version: _,
        image,
        pipeline,
        resources,
    } = doc.to_v1();

    db.set_abi(resolve_abi(&image));

    let (names, d) = resolve_names(db, &pipeline, &resources);
    db.set_names((names.clone(), d));

    for (name, id) in names {
        if let HirId::Node(id) = id {
            let stage = &pipeline[name.as_str()];
            let resolved = resolve_args(db, &name, stage.args());
            db.set_arguments(id, resolved);

            let resolved = resolve_inputs(db, &name, stage.inputs());
            db.set_inputs(id, resolved);
        }
    }
}

/// The database containing Rune's high-level intermediate representation.
///
/// # Usage
///
/// If you are using the YAML frontend, you will typicaly use
/// [`populate_from_document()`] function to populate the [`HirDB`] using a
/// parsed `Runefile.yml`.
///
/// If you are implementing your own frontend (e.g. a canvas in the browser)
/// there is no `Runefile.yml` per-se, so you will want to call the setters
/// yourself.
#[salsa::query_group(HirDBStorage)]
pub trait HirDB {
    #[salsa::input]
    fn abi(&self) -> (Abi, Diagnostics);

    #[salsa::input]
    fn names(&self) -> (OrdMap<Text, HirId>, Diagnostics);

    /// An interned [`Node`].
    #[salsa::interned]
    fn node(&self, node: Node) -> NodeId;

    #[salsa::input]
    fn inputs(&self, node: NodeId) -> (Vector<Option<Input>>, Diagnostics);

    /// An interned argument.
    #[salsa::interned]
    fn argument(&self, arg: Argument) -> ArgumentId;

    /// Retrieve the arguments associated with a [`Node`].
    #[salsa::input]
    fn arguments(
        &self,
        node_id: NodeId,
    ) -> (OrdMap<Text, ArgumentId>, Diagnostics);

    #[salsa::interned]
    fn resource(&self, res: Resource) -> ResourceId;

    /// All the [`Diagnostics`] that were encountered while populating the
    /// [`HirDB`].
    fn lowering_diagnostics(&self) -> Diagnostics;
}

#[tracing::instrument(skip(db, pipeline, resources))]
fn resolve_names(
    db: &dyn HirDB,
    pipeline: &IndexMap<String, parse::Stage>,
    resources: &IndexMap<String, parse::ResourceDeclaration>,
) -> (OrdMap<Text, HirId>, Diagnostics) {
    let mut names: OrdMap<Text, HirId> = OrdMap::new();
    let mut diags = Diagnostics::new();

    for (name, stage) in pipeline {
        let name = Text::new(name.as_str());
        let (node, d) = resolve_node(db, stage);
        diags.extend(d);
        let id = db.node(node).into();

        if let Some(original) = names.insert(name.clone(), id) {
            diags.push(DuplicateName::new(original, id, name).as_diagnostic());
        }
    }

    for (name, resource) in resources {
        let name = Text::new(name.as_str());
        let (resource, path_and_inline_defined) = resolve_resource(resource);
        let id = db.resource(resource);
        if path_and_inline_defined {
            let diag = PathAndInlineNotAllowed::new(name.as_str(), id);
            diags.push(diag.as_diagnostic());
        }
        let id = HirId::from(id);
        if let Some(original) = names.insert(name.clone(), id) {
            diags.push(DuplicateName::new(original, id, name).as_diagnostic());
        }
    }

    (names, diags)
}

fn resolve_node(db: &dyn HirDB, stage: &parse::Stage) -> (Node, Diagnostics) {
    let mut diags = Diagnostics::new();

    let (kind, identifier) = match stage {
        parse::Stage::Model(m) => {
            let (value, d) = resolve_resource_or_string(db, &m.model);
            diags.extend(d);
            (NodeKind::Model, value)
        },
        parse::Stage::ProcBlock(p) => {
            // TODO: pass the path along as-is instead of marshalling it via a
            // string
            (
                NodeKind::ProcBlock,
                ResourceOrText::Text(p.proc_block.to_string().into()),
            )
        },
        parse::Stage::Capability(c) => {
            (NodeKind::Input, ResourceOrText::text(&c.capability))
        },
        parse::Stage::Out(o) => {
            (NodeKind::Output, ResourceOrText::text(&o.out))
        },
    };

    let node = Node {
        kind,
        identifier,
        outputs: stage.output_types().iter().cloned().collect(),
    };
    (node, diags)
}

#[tracing::instrument(level = "debug", skip(db, args))]
fn resolve_args(
    db: &mut dyn HirDB,
    node_name: &Text,
    args: &IndexMap<String, parse::Argument>,
) -> (OrdMap<Text, ArgumentId>, Diagnostics) {
    let mut argument_names = OrdMap::new();
    let mut diags = Diagnostics::new();

    for (name, value) in args {
        let name = Text::new(name.as_str());
        let (value, d) = resolve_resource_or_string(db, value);
        diags.extend(d);
        let arg = Argument { value };
        let id = db.argument(arg);
        argument_names.insert(name, id);
    }

    (argument_names, diags)
}

#[tracing::instrument(level = "debug", skip(db, inputs))]
fn resolve_inputs(
    db: &dyn HirDB,
    name: &str,
    inputs: &[crate::parse::Input],
) -> (Vector<Option<Input>>, Diagnostics) {
    let mut resolved = Vector::new();
    let mut diags = Diagnostics::new();

    let (names, _) = db.names();

    for input in inputs {
        match names.get(input.name.as_str()).copied() {
            Some(HirId::Node(id)) => {
                let index = input.index.unwrap_or(0);
                resolved.push_back(Some(Input { node: id, index }));
            },
            Some(HirId::Resource(id)) => {
                let diag =
                    ResourceUsedAsInput::new(input.clone(), name.into(), id);
                diags.push(diag.as_diagnostic());
                resolved.push_back(None);
            },
            None => {
                let diag = UnknownInput::new(input.clone());
                diags.push(diag.as_diagnostic());
                resolved.push_back(None);
            },
        }
    }

    (resolved, diags)
}

#[tracing::instrument(level = "debug", skip(db))]
fn resolve_resource_or_string(
    db: &dyn HirDB,
    value: &parse::ResourceOrString,
) -> (ResourceOrText, Diagnostics) {
    match value {
        parse::ResourceOrString::String(s) => {
            (ResourceOrText::Text(s.as_str().into()), Diagnostics::new())
        },
        parse::ResourceOrString::Resource(r) => {
            let (names, _) = db.names();

            match names.get(r.as_str()).copied() {
                Some(HirId::Resource(id)) => {
                    (ResourceOrText::Resource(id), Diagnostics::new())
                },
                Some(HirId::Node(_)) => {
                    let diag = NotAResource { name: r.clone() };
                    (
                        ResourceOrText::Error,
                        Diagnostics::one(diag.as_diagnostic()),
                    )
                },
                None => {
                    let diag = UnknownResource { name: r.clone() };
                    (
                        ResourceOrText::Error,
                        Diagnostics::one(diag.as_diagnostic()),
                    )
                },
            }
        },
    }
}

#[tracing::instrument(level = "debug", skip(decl))]
fn resolve_resource(decl: &parse::ResourceDeclaration) -> (Resource, bool) {
    let parse::ResourceDeclaration { inline, path, ty } = decl;

    let mut path_and_inline_defined = false;

    let default_value = match (inline.as_deref(), path.as_deref()) {
        (Some(inline), None) => Some(ResourceSource::inline(inline)),
        (None, Some(path)) => Some(ResourceSource::from_disk(path)),
        (Some(_), Some(_)) => {
            path_and_inline_defined = true;
            None
        },
        (None, None) => None,
    };

    let resource = Resource {
        default_value,
        ty: *ty,
    };

    (resource, path_and_inline_defined)
}

#[tracing::instrument(level = "debug", skip(db))]
fn lowering_diagnostics(db: &dyn HirDB) -> Diagnostics {
    let mut diagnostics = Diagnostics::new();

    let (_, diags) = db.abi();
    diagnostics.extend(diags);

    let (names, diags) = db.names();
    diagnostics.extend(diags);

    for (_, id) in names {
        match id {
            HirId::Node(id) => {
                let (_, diags) = db.arguments(id);
                diagnostics.extend(diags);
                let (_, diags) = db.inputs(id);
                diagnostics.extend(diags);
            },
            HirId::Resource(_) => {},
        }
    }

    diagnostics
}

#[tracing::instrument(level = "debug", skip(image))]
fn resolve_abi(image: &parse::Image) -> (Abi, Diagnostics) {
    let parse::Path {
        base,
        sub_path,
        version,
    } = &image.0;
    match (base.as_str(), sub_path.as_deref(), version.as_deref()) {
        ("runicos/base", None, None) => (Abi::V0, Diagnostics::new()),
        _ => {
            let diag = UnknownAbi {
                image: image.clone(),
            };
            (Abi::V0, diag.as_diagnostic().into())
        },
    }
}

#[cfg(test)]
mod tests {
    use salsa::Database;

    use super::*;

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct SerializedHir {
        diags: Diagnostics,
        abi: Abi,
        names: OrdMap<Text, HirId>,
        nodes: OrdMap<NodeId, Node>,
        inputs: OrdMap<NodeId, Vector<Option<Input>>>,
        arguments: OrdMap<NodeId, OrdMap<Text, Argument>>,
        resources: OrdMap<ResourceId, Resource>,
    }

    fn load_state(db: &dyn HirDB) -> SerializedHir {
        let (abi, _) = db.abi();
        let (names, _) = db.names();

        let mut nodes = OrdMap::new();
        let mut resources = OrdMap::new();
        let mut arguments = OrdMap::new();
        let mut inputs = OrdMap::new();

        for &id in names.values() {
            match id {
                HirId::Node(id) => {
                    nodes.insert(id, db.lookup_node(id));
                    let (args, _) = db.arguments(id);
                    let args = args
                        .into_iter()
                        .map(|(name, id)| (name, db.lookup_argument(id)))
                        .collect();
                    arguments.insert(id, args);
                    let (node_inputs, _) = db.inputs(id);
                    inputs.insert(id, node_inputs);
                },
                HirId::Resource(id) => {
                    resources.insert(id, db.lookup_resource(id));
                },
            }
        }

        let diags = db.lowering_diagnostics();

        SerializedHir {
            diags,
            abi,
            names,
            nodes,
            arguments,
            resources,
            inputs,
        }
    }

    #[derive(Default)]
    #[salsa::database(HirDBStorage)]
    struct DB {
        storage: salsa::Storage<Self>,
    }

    impl Database for DB {}

    macro_rules! expect_diagnostic {
        ($name:ident, $diagnostic:ty, $src:literal) => {
            #[test]
            fn $name() {
                let doc = crate::parse::parse_runefile($src).unwrap();
                let mut db = DB::default();

                populate_from_document(&mut db, doc);

                let diags = db.lowering_diagnostics();

                println!("{:#?}", diags);
                assert_eq!(diags.len(), 1);
                let diag = diags.iter().next().unwrap();
                assert_eq!(diag.meta, Some(<$diagnostic>::meta()));
            }
        };
    }

    expect_diagnostic!(
        duplicate_names,
        DuplicateName,
        r#"
            version: 1
            image: "runicos/base"
            pipeline:
                first:
                    capability: RAW
            resources:
                first: {}
        "#
    );

    expect_diagnostic!(
        resource_used_as_input,
        ResourceUsedAsInput,
        r#"
            version: 1
            image: "runicos/base"
            pipeline:
                sine:
                    model: sine.tflite
                    inputs:
                        - res
            resources:
                res: {}
        "#
    );

    expect_diagnostic!(
        path_and_inline_not_allowed,
        PathAndInlineNotAllowed,
        r#"
            version: 1
            image: "runicos/base"
            pipeline: {}
            resources:
                res:
                    path: ./foo.txt
                    inline: bar
        "#
    );

    expect_diagnostic!(
        unknown_abi,
        UnknownAbi,
        r#"
            version: 1
            image: something-else
            pipeline: {}
            resources: {}
        "#
    );

    #[test]
    fn populate_gesture() {
        let src = include_str!("../../../../examples/gesture/Runefile.yml");
        let doc = crate::parse::parse_runefile(src).unwrap();
        let mut db = DB::default();

        populate_from_document(&mut db, doc);

        let diags = db.lowering_diagnostics();
        assert!(diags.is_empty());
        let state = load_state(&db);
        insta::assert_yaml_snapshot!(state);
    }
}

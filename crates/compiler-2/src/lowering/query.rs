use im::{HashMap, HashSet, Vector};
use indexmap::IndexMap;

use crate::{
    diagnostics::{
        AsDiagnostic, Diagnostic, DiagnosticMetadata, Diagnostics, Severity,
    },
    lowering::{
        Abi, Argument, ArgumentId, DuplicateName, Identifiers, Node, NodeId,
        PathAndInlineNotAllowed, Resource, ResourceId, ResourceOrText,
        ResourceSource, UnknownResource,
    },
    parse, Text,
};

/// Populate a [`HirDB`] using a [`parse::Document`].
#[must_use]
#[tracing::instrument(skip(db, doc))]
pub fn populate_from_document(db: &mut dyn HirDB, doc: parse::Document) {
    let mut ids = Identifiers::new();

    let parse::DocumentV1 {
        // Only used to switch between Runefile.yml formats
        version: _,
        image,
        pipeline,
        resources,
    } = doc.to_v1();

    db.set_abi(resolve_abi(&image));

    let node_names = resolve_node_names(&pipeline, &mut ids);
    db.set_node_names(node_names);

    let resource_names = resolve_resource_names(&resources, &mut ids);
    db.set_resource_names(resource_names);

    for (name, id) in db.resource_names() {
        let decl = &resources[name.as_str()];
        db.set_resource(id, resolve_resource(&name, id, decl));
    }

    for (name, id) in db.node_names() {
        let node = &pipeline[name.as_str()];
        resolve_args(db, &name, id, node.args());
    }
}

/// The database containing Rune's high-level intermediate representation.
#[salsa::query_group(HirDBStorage)]
pub trait HirDB {
    #[salsa::input]
    fn abi(&self) -> (Abi, Diagnostics);
    #[salsa::input]
    fn node_names(&self) -> HashMap<Text, NodeId>;
    #[salsa::input]
    fn node(&self, id: NodeId) -> (Node, Diagnostics);

    /// Retrieve the arguments associated with a [`Node`].
    #[salsa::input]
    fn arguments(&self, node_id: NodeId) -> HashMap<Text, ArgumentId>;
    #[salsa::input]
    fn argument(&self, id: ArgumentId) -> Result<Argument, Diagnostic>;

    #[salsa::input]
    fn resource_names(&self) -> HashMap<Text, ResourceId>;
    #[salsa::input]
    fn resource(&self, id: ResourceId) -> (Resource, Diagnostics);

    /// Get [`DuplicateName`]s for any names that are duplicated between
    /// [`HirDB::node_names()`] and [`HirDB::resource_names()`].
    fn duplicate_names(&self) -> Vector<DuplicateName>;

    /// All the [`Diagnostics`] that were encountered while populating the
    /// [`HirDB`].
    fn lowering_diagnostics(&self) -> Diagnostics;
}

#[tracing::instrument(level = "debug", skip(db, args, id))]
fn resolve_args(
    db: &mut dyn HirDB,
    node_name: &Text,
    id: NodeId,
    args: &IndexMap<String, parse::Argument>,
) {
    let mut argument_names = HashMap::new();

    for (name, value) in args {
        let name = Text::new(name.as_str());
        let arg_id = ArgumentId::new(id, name.clone());
        argument_names.insert(name, arg_id.clone());

        let arg = resolve_resource_or_string(db, value)
            .map(|value| Argument { value })
            .map_err(|e| e.as_diagnostic());

        db.set_argument(arg_id, arg);
    }

    db.set_arguments(id, argument_names);
}

#[tracing::instrument(level = "debug", skip(db))]
fn resolve_resource_or_string(
    db: &dyn HirDB,
    value: &parse::ResourceOrString,
) -> Result<ResourceOrText, UnknownResource> {
    match value {
        parse::ResourceOrString::String(s) => {
            Ok(ResourceOrText::Text(s.as_str().into()))
        },
        parse::ResourceOrString::Resource(r) => {
            let resources = db.resource_names();

            resources
                .get(r.as_str())
                .copied()
                .map(|id| ResourceOrText::Resource(id))
                .ok_or_else(|| UnknownResource { name: r.clone() })
        },
    }
}

#[tracing::instrument(level = "debug", skip(decl))]
fn resolve_resource(
    name: &str,
    id: ResourceId,
    decl: &parse::ResourceDeclaration,
) -> (Resource, Diagnostics) {
    let parse::ResourceDeclaration { inline, path, ty } = decl;

    let mut diags = Diagnostics::new();

    let default_value = match (inline.as_deref(), path.as_deref()) {
        (Some(inline), None) => Some(ResourceSource::inline(inline)),
        (None, Some(path)) => Some(ResourceSource::from_disk(path)),
        (Some(_), Some(_)) => {
            diags.push(PathAndInlineNotAllowed::new(name, id).as_diagnostic());
            None
        },
        (None, None) => None,
    };

    let resource = Resource {
        default_value,
        ty: *ty,
    };
    (resource, diags)
}

#[tracing::instrument(level = "debug", skip(db))]
fn duplicate_names(db: &dyn HirDB) -> Vector<DuplicateName> {
    let nodes = db.node_names();
    let resources = db.resource_names();

    let node_names: HashSet<Text> = nodes.keys().cloned().collect();
    let resource_names: HashSet<Text> = resources.keys().cloned().collect();

    node_names
        .intersection(resource_names)
        .into_iter()
        .map(|name| {
            let (_, &node_id) = nodes
                .iter()
                .find(|(n, _)| n.as_str() == name.as_str())
                .unwrap();
            let (_, &resource_id) = resources
                .iter()
                .find(|(n, _)| n.as_str() == name.as_str())
                .unwrap();

            DuplicateName {
                name,
                node_id,
                resource_id,
            }
        })
        .collect()
}

#[tracing::instrument(level = "debug", skip(db))]
fn lowering_diagnostics(db: &dyn HirDB) -> Diagnostics {
    let mut diagnostics = Diagnostics::new();

    let (_, diags) = db.abi();
    diagnostics.extend(diags);

    for (_, id) in db.node_names() {
        let (_, diags) = db.node(id);
        diagnostics.extend(diags);

        for (_, arg_id) in db.arguments(id) {
            if let Err(diag) = db.argument(arg_id) {
                diagnostics.push(diag);
            }
        }
    }

    for (_, id) in db.resource_names() {
        let (_, diags) = db.resource(id);
        diagnostics.extend(diags);
    }

    diagnostics
        .extend(db.duplicate_names().into_iter().map(|d| d.as_diagnostic()));

    diagnostics
}

#[tracing::instrument(level = "debug")]
fn resolve_resource_names(
    resources: &IndexMap<String, parse::ResourceDeclaration>,
    ids: &mut Identifiers,
) -> HashMap<Text, ResourceId> {
    resources
        .keys()
        .map(|name| (Text::new(name.as_str()), ids.resource()))
        .collect()
}

#[tracing::instrument(level = "debug")]
fn resolve_node_names(
    pipeline: &IndexMap<String, parse::Stage>,
    ids: &mut Identifiers,
) -> HashMap<Text, NodeId> {
    pipeline
        .keys()
        .map(|name| (Text::new(name.as_str()), ids.node()))
        .collect()
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
        _ => (
            Abi::V1,
            Diagnostic::new(
                Severity::Warning,
                format!("Unknown ABI, \"{}\"", image),
            )
            .into(),
        ),
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
)]
#[error("Unknown ABI, \"{}\"", image)]
struct UnknownAbi {
    image: parse::Image,
}

impl AsDiagnostic for UnknownAbi {
    fn meta() -> DiagnosticMetadata { DiagnosticMetadata::new("Unknown ABI") }
}

#[cfg(test)]
mod tests {
    use salsa::Database;

    use super::*;
    use crate::lowering::DuplicateName;

    #[derive(Default)]
    #[salsa::database(HirDBStorage)]
    struct DB {
        storage: salsa::Storage<Self>,
    }

    impl Database for DB {}

    #[test]
    fn duplicate_names() {
        let mut ids = Identifiers::new();
        let mut nodes = HashMap::new();
        nodes.insert(Text::new("a"), ids.node());
        nodes.insert(Text::new("b"), ids.node());
        let mut resources = HashMap::new();
        resources.insert(Text::new("a"), ids.resource());
        resources.insert(Text::new("c"), ids.resource());

        let mut db = DB::default();
        db.set_node_names(nodes.clone());
        db.set_resource_names(resources.clone());

        let errors = db.duplicate_names();

        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0],
            DuplicateName {
                resource_id: resources["a"],
                node_id: nodes["a"],
                name: "a".into()
            }
        );
    }
}

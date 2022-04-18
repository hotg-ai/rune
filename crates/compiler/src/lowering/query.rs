use std::{num::NonZeroU32, sync::Arc};

use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use im::HashMap;

use crate::{
    inputs::FileSystem,
    lowering::{Inputs, Outputs, Resource, ResourceSource},
    parse::{DocumentV1, Image, ResourceDeclaration, ResourceOrString},
    Diagnostics,
};

/// This is typically populated by something like [`apply_document()`].
#[salsa::query_group(LoweringInputsGroup)]
pub trait Ast {
    #[salsa::input]
    fn image(&self) -> Image;

    #[salsa::input]
    fn nodes(&self) -> HashMap<Arc<str>, NodeId>;
    #[salsa::input]
    fn node(&self, id: NodeId) -> Node;
    #[salsa::input]
    fn node_inputs(&self, id: NodeId) -> Inputs;
    #[salsa::input]
    fn node_outputs(&self, id: NodeId) -> Outputs;

    #[salsa::input]
    fn resources(&self) -> HashMap<Arc<str>, ResourceId>;
    #[salsa::input]
    fn resource(&self, id: ResourceId) -> Resource;
}

#[salsa::query_group(LoweringGroup)]
pub trait Lowering: Ast + FileSystem {}

#[must_use = "Diagnostics should always be handled"]
pub fn apply_document(db: &mut dyn Ast, doc: &DocumentV1) -> Diagnostics {
    let DocumentV1 {
        version: _,
        image,
        pipeline,
        resources,
    } = doc;

    let mut diags = Diagnostics::new();

    db.set_image(image.clone());

    let mut ids = Counter::new();

    set_nodes(pipeline, &mut ids, db);
    set_resources(resources, &mut ids, db, &mut diags);

    diags
}

fn set_resources(
    resources: &indexmap::IndexMap<String, ResourceDeclaration>,
    ids: &mut Counter,
    db: &mut dyn Ast,
    diags: &mut Diagnostics,
) {
    let mut res = HashMap::new();

    for (name, decl) in resources {
        let id = ids.resource_id();

        let source = match (decl.inline.as_ref(), decl.path.as_ref()) {
            (Some(inline), None) => {
                Some(ResourceSource::Inline(inline.clone()))
            },
            (None, Some(path)) => Some(ResourceSource::FromDisk(path.into())),
            (None, None) => None,
            (Some(_), Some(_)) => {
                let diag = PathXorInlineResourceDiagnostic {
                    name: name.to_string(),
                    span: decl.span(),
                };
                diags.push(diag.into_codespan_diagnostic());

                continue;
            },
        };

        let resource = Resource {
            default_value: source,
            ty: decl.ty,
        };
        res.insert(Arc::from(name.as_str()), id);
        db.set_resource(id, resource);
    }

    db.set_resources(res);
}

#[derive(Debug, Clone, thiserror::Error, miette::Diagnostic)]
#[error("A resource can't specify both \"path\" and \"inline\" values")]
#[diagnostic(code("E002"))]
pub struct PathXorInlineResourceDiagnostic {
    pub name: String,
    pub span: Span,
}

impl PathXorInlineResourceDiagnostic {
    fn into_codespan_diagnostic(&self) -> Diagnostic<()> {
        let msg = format!(
            "The resource \"{}\" can't specify both a \"path\" and \"inline\" \
             default value",
            self.name
        );

        Diagnostic::error()
            .with_message(msg)
            .with_labels(vec![Label::primary((), self.span)])
    }
}

fn set_nodes(
    pipeline: &indexmap::IndexMap<String, crate::parse::Stage>,
    ids: &mut Counter,
    db: &mut dyn Ast,
) {
    let mut nodes = HashMap::new();
    for (name, stage) in pipeline {
        let id = ids.node_id();
        let name: Arc<str> = name.as_str().into();
        nodes.insert(name, id);
        db.set_node(id, Node::from_stage(stage));
    }
    db.set_nodes(nodes);
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Node {
    kind: NodeKind,
    identifier: ResourceOrString,
    arguments: HashMap<Arc<str>, ResourceOrString>,
}

impl Node {
    fn from_stage(stage: &crate::parse::Stage) -> Self {
        let (kind, identifier) = match stage {
            crate::parse::Stage::Model(m) => (NodeKind::Model, m.model.clone()),
            crate::parse::Stage::ProcBlock(_) => todo!(),
            crate::parse::Stage::Capability(_) => todo!(),
            crate::parse::Stage::Out(_) => todo!(),
        };

        let arguments = stage
            .args()
            .iter()
            .map(|(k, v)| (Arc::from(k.as_str()), v.0.clone()))
            .collect();

        Node {
            kind,
            identifier,
            arguments,
        }
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum NodeKind {
    Capability,
    Model,
    ProcBlock,
    Output,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(transparent)]
pub struct NodeId(NonZeroU32);

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(transparent)]
pub struct ResourceId(NonZeroU32);

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct ArgumentId {
    node: NodeId,
    name: Arc<str>,
}

#[derive(Debug)]
struct Counter(u32);

impl Counter {
    fn new() -> Self { Counter(0) }

    fn next(&mut self) -> NonZeroU32 {
        self.0 += 1;
        NonZeroU32::new(self.0).expect("Unreachable")
    }

    fn node_id(&mut self) -> NodeId { NodeId(self.next()) }

    fn resource_id(&mut self) -> ResourceId { ResourceId(self.next()) }
}

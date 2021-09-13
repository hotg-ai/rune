use codespan::Span;
use codespan_reporting::diagnostic::Diagnostic;
use indexmap::IndexMap;
use crate::{
    Diagnostics,
    hir::{HirId, NameTable, Resource, ResourceSource},
    passes::helpers,
    utils::HirIds,
    yaml::ResourceDeclaration,
};

pub(crate) fn run(
    diags: &mut Diagnostics,
    ids: &mut HirIds,
    resource_decls: &IndexMap<String, ResourceDeclaration>,
    resources: &mut IndexMap<HirId, Resource>,
    spans: &IndexMap<HirId, Span>,
    names: &mut NameTable,
) {
    for (name, declaration) in resource_decls {
        let source = match declaration {
            ResourceDeclaration {
                inline: Some(inline),
                path: None,
                ..
            } => Some(ResourceSource::Inline(inline.clone())),
            ResourceDeclaration {
                inline: None,
                path: Some(path),
                ..
            } => Some(ResourceSource::FromDisk(path.into())),
            ResourceDeclaration {
                inline: None,
                path: None,
                ..
            } => None,
            ResourceDeclaration {
                inline: Some(_),
                path: Some(_),
                ..
            } => {
                let diag = Diagnostic::error().with_message(format!("The resource \"{}\" can't specify both a \"path\" and \"inline\" value", name));
                diags.push(diag);
                continue;
            },
        };
        let id = ids.next();
        let resource = Resource {
            source,
            ty: declaration.ty,
        };
        helpers::register_name(name, id, resource.span(), spans, names, diags);
        resources.insert(id, resource);
    }
}

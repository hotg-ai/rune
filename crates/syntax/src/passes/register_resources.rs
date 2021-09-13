use codespan_reporting::diagnostic::Diagnostic;
use indexmap::IndexMap;

use crate::{
    hir::{Resource, ResourceSource},
    yaml::ResourceDeclaration,
};

use super::Context;

pub(crate) fn run(
    ctx: &mut Context<'_>,
    resources: &IndexMap<String, ResourceDeclaration>,
) {
    for (name, declaration) in resources {
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
                ctx.diags.push(diag);
                continue;
            },
        };
        let id = ctx.ids.next();
        let resource = Resource {
            source,
            ty: declaration.ty,
        };
        ctx.register_name(name, id, resource.span());
        ctx.rune.register_resource(id, resource);
    }
}

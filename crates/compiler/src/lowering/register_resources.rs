use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use legion::systems::CommandBuffer;

use crate::{
    lowering::{NameTable, Resource, ResourceSource},
    parse::DocumentV1,
    Diagnostics,
};

/// Register all the [`Resource`]s in a [`DocumentV1`].
#[legion::system]
pub(crate) fn run(
    cmd: &mut CommandBuffer,
    #[resource] diags: &mut Diagnostics,
    #[resource] doc: &mut DocumentV1,
    #[resource] names: &NameTable,
) {
    for (name, decl) in &doc.resources {
        let ent = match names.get(name) {
            Some(&e) => e,
            None => {
                // there was probably a duplicate name error in an earlier pass
                // so this resource wasn't added to the name table.
                continue;
            },
        };

        let source = match (decl.inline.as_ref(), decl.path.as_ref()) {
            (Some(inline), None) => {
                Some(ResourceSource::Inline(inline.clone()))
            },
            (None, Some(path)) => Some(ResourceSource::FromDisk(path.into())),
            (None, None) => None,
            (Some(_), Some(_)) => {
                diags.push(path_and_inline_defined_diagnostic(
                    name,
                    decl.span(),
                ));

                continue;
            },
        };

        let resource = Resource {
            default_value: source,
            ty: decl.ty,
        };

        cmd.add_component(ent, resource);
    }
}

fn path_and_inline_defined_diagnostic(
    name: &str,
    span: Span,
) -> Diagnostic<()> {
    let msg = format!(
        "The resource \"{}\" can't specify both a \"path\" and \"inline\" \
         default value",
        name
    );

    Diagnostic::error()
        .with_message(msg)
        .with_labels(vec![Label::primary((), span)])
}

#[cfg(never)]
mod tests {
    use legion::{IntoQuery, Resources, World};

    use super::*;
    use crate::{
        lowering,
        lowering::Name,
        parse::{ResourceDeclaration, ResourceType},
        phases::Phase,
        BuildContext,
    };

    fn doc() -> DocumentV1 {
        DocumentV1 {
            version: 1,
            image: "img".parse().unwrap(),
            pipeline: Default::default(),
            resources: map! {
                inline_string: ResourceDeclaration {
                    inline: Some("inline".to_string()),
                    path: None,
                    ty: ResourceType::String,
                },
                path_bytes: ResourceDeclaration {
                    inline: None,
                    path: Some("data.bin".to_string()),
                    ty: ResourceType::Binary,
                },
                no_defaults: ResourceDeclaration {
                    ty: ResourceType::Binary,
                    ..Default::default()
                },
                error: ResourceDeclaration {
                    inline: Some("inline".to_string()),
                    path: Some("data.bin".to_string()),
                    ..Default::default()
                }
            },
        }
    }

    #[test]
    fn all_resources_are_registered() {
        let mut world = World::default();
        let mut res = Resources::default();
        res.insert(BuildContext::from_doc(doc().into()));
        res.insert(NameTable::default());
        let should_be = vec![
            (
                Name::from("inline_string"),
                Resource {
                    default_value: Some(ResourceSource::Inline(
                        "inline".to_string(),
                    )),
                    ty: ResourceType::String,
                },
            ),
            (
                Name::from("path_bytes"),
                Resource {
                    default_value: Some(ResourceSource::FromDisk(
                        "data.bin".into(),
                    )),
                    ty: ResourceType::Binary,
                },
            ),
            (
                Name::from("no_defaults"),
                Resource {
                    default_value: None,
                    ty: ResourceType::Binary,
                },
            ),
        ];
        crate::parse::phase().run(&mut world, &mut res);

        Phase::new()
            .and_then(lowering::register_names::run_system)
            .and_then(lowering::update_nametable::run_system)
            .and_then(run_system)
            .run(&mut world, &mut res);

        let resources: Vec<_> = <(&Name, &Resource)>::query()
            .iter(&world)
            .map(|(n, r)| (n.clone(), r.clone()))
            .collect();
        assert_eq!(resources, should_be);

        let diags = res.get::<Diagnostics>().unwrap();
        let diags: Vec<_> = diags.iter().collect();
        assert_eq!(diags.len(), 1);
        assert_eq!(
            diags[0].message,
            "The resource \"error\" can't specify both a \"path\" and \
             \"inline\" default value",
        );
    }
}

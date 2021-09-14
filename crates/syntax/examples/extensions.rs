//! An example showing how you might hook into the Rune build process.
//!
//! This example achieves a couple things:
//!
//! 1. After the YAML document is analysed we emit a warning for every [`Model`]
//!    that it includes
//! 2. We implement dotenv-like functionality by checking if any resources (e.g.
//!    `foo`) have the corresponding environment variable set (e.g. `$FOO`). If
//!    so, the resource's [`ResourceData`] component is overridden with its
//!    value.
//! 3. Print out some any diagnostics at the end so we can see the effects from
//!    step 1.

use codespan_reporting::diagnostic::Diagnostic;
use hotg_rune_syntax::{
    BuildContext, Diagnostics,
    hir::{Model, Name, Resource, ResourceData},
    hooks::{
        AfterLoweringContext, AfterTypeCheckingContext, Continuation, Hooks,
    },
};
use legion::{Entity, IntoQuery, component, systems::CommandBuffer};

fn main() {
    let directory = std::env::args().nth(1).expect("Usage: ./extensions <dir>");
    let build_ctx = BuildContext::for_directory(directory)
        .expect("Couldn't read the Runefile");

    let mut hooks = CustomHooks::default();

    let (_world, res) = hotg_rune_syntax::build(build_ctx, &mut hooks);

    // Print out all diagnostics. Normally you'd use the codespan_reporting
    // crate, but println!() is good enough for now.
    let diags = res.get::<Diagnostics>().unwrap();

    println!("Printing {} diagnostics...", diags.len());
    for diag in diags.iter() {
        println!("  {:?}: {}", diag.severity, diag.message);
    }
}

#[derive(Debug, Default)]
struct CustomHooks {}

impl Hooks for CustomHooks {
    fn after_lowering(
        &mut self,
        ctx: &mut dyn AfterLoweringContext,
    ) -> Continuation {
        warn_about_every_model_in_the_rune(ctx);
        Continuation::Continue
    }

    fn after_type_checking(
        &mut self,
        ctx: &mut dyn AfterTypeCheckingContext,
    ) -> Continuation {
        dotenv(ctx);
        Continuation::Continue
    }
}

fn warn_about_every_model_in_the_rune(ctx: &mut dyn AfterLoweringContext) {
    let mut diags = ctx.diagnostics_mut();
    let mut model_names = <&Name>::query().filter(component::<Model>());

    for name in model_names.iter(ctx.world()) {
        let msg = format!("The Rune contains a model called \"{}\"", name);
        diags.push(Diagnostic::warning().with_message(msg));
    }
}

/// Implement `dotenv`-like behaviour by looking for the environment variable
/// that corresponds to a particular [`Resource`] and setting its
/// [`ResourceData`] if that variable is set.
fn dotenv(ctx: &mut dyn AfterTypeCheckingContext) {
    let (world, res) = ctx.world_and_resources();

    let mut cmd = CommandBuffer::new(world);

    // create a query which will look for all named entities with a "Resource"
    // component.
    let mut query = <(Entity, &Name)>::query().filter(component::<Resource>());

    for (&ent, name) in query.iter(world) {
        let variable_name = name.to_uppercase();

        if let Ok(value) = std::env::var(variable_name) {
            println!(
                "Overriding the \"{}\" resource and setting it to \"{}\"",
                name, value
            );
            cmd.add_component(ent, ResourceData::from(value));
        }
    }

    cmd.flush(world, res);
}

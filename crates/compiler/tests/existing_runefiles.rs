use codespan_reporting::{
    files::SimpleFile,
    term::{termcolor::Buffer, Config},
};
use serde_json::Value;
use jsonschema::JSONSchema;
use hotg_rune_compiler::{
    Diagnostics,
    parse::Document,
    hooks::{
        Hooks, AfterTypeCheckingContext, AfterCodegenContext, Continuation,
    },
    codegen::RuneVersion,
    BuildContext, Verbosity, FeatureFlags,
};

macro_rules! parse_and_analyse {
    ($example:ident) => {
        mod $example {
            use super::*;

            const PATH: &str = concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../examples/",
                stringify!($example),
            );
            const SRC: &str = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../examples/",
                stringify!($example),
                "/Runefile.yml"
            ));

            #[test]
            fn parse() { let _ = Document::parse(SRC).unwrap(); }

            #[derive(Debug, Copy, Clone, PartialEq)]
            enum Phase {
                TypeCheck,
                Codegen,
            }

            struct AbortAfterPhase {
                diags: Diagnostics,
                phase: Phase,
            }

            impl AbortAfterPhase {
                fn new(phase: Phase) -> Self {
                    AbortAfterPhase {
                        phase,
                        diags: Diagnostics::new(),
                    }
                }

                fn maybe_abort(
                    &mut self,
                    phase: Phase,
                    diags: &Diagnostics,
                ) -> Continuation {
                    if phase == self.phase {
                        for diag in diags.iter() {
                            self.diags.push(diag.clone());
                        }
                        Continuation::Halt
                    } else {
                        Continuation::Continue
                    }
                }
            }

            impl Hooks for AbortAfterPhase {
                fn after_type_checking(
                    &mut self,
                    ctx: &mut dyn AfterTypeCheckingContext,
                ) -> Continuation {
                    self.maybe_abort(Phase::TypeCheck, &ctx.diagnostics())
                }

                fn after_codegen(
                    &mut self,
                    ctx: &mut dyn AfterCodegenContext,
                ) -> Continuation {
                    self.maybe_abort(Phase::Codegen, &ctx.diagnostics())
                }
            }

            fn handle_diagnostics(
                file: &SimpleFile<&'static str, &'static str>,
                diags: &Diagnostics,
            ) {
                let mut writer = Buffer::no_color();
                let config = Config::default();

                for diag in diags {
                    codespan_reporting::term::emit(
                        &mut writer,
                        &config,
                        file,
                        diag,
                    )
                    .unwrap();
                }

                if diags.has_errors() {
                    panic!("{}", String::from_utf8_lossy(writer.as_slice()));
                }
            }

            fn build_context() -> BuildContext {
                BuildContext {
                    name: stringify!($example).to_string(),
                    runefile: SRC.to_string(),
                    working_directory: PATH.into(),
                    current_directory: PATH.into(),
                    optimized: false,
                    verbosity: Verbosity::Normal,
                    rune_version: Some(RuneVersion {
                        version: env!("CARGO_PKG_VERSION").to_string(),
                    }),
                }
            }

            #[test]
            fn analyse() {
                let file = SimpleFile::new("Runefile", SRC);
                let ctx = build_context();
                let mut hooks = AbortAfterPhase::new(Phase::TypeCheck);

                hotg_rune_compiler::build_with_hooks(
                    ctx,
                    FeatureFlags::development(),
                    &mut hooks,
                );

                handle_diagnostics(&file, &hooks.diags);
            }

            #[test]
            fn codegen() {
                let file = SimpleFile::new("Runefile", SRC);
                let ctx = build_context();
                let mut hooks = AbortAfterPhase::new(Phase::Codegen);

                hotg_rune_compiler::build_with_hooks(
                    ctx,
                    FeatureFlags::development(),
                    &mut hooks,
                );

                handle_diagnostics(&file, &hooks.diags);
            }

            #[test]
            fn validate_against_yaml_schema() {
                let document: Value = serde_yaml::from_str(SRC).unwrap();

                let schema = schemars::schema_for!(Document);
                let schema = serde_json::to_value(&schema).unwrap();

                let compiled_schema =
                    JSONSchema::options().compile(&schema).unwrap();

                let result = compiled_schema.validate(&document);
                if let Err(errors) = result {
                    for error in errors {
                        println!("Validation error: {}", error);
                        println!("Instance path: {}", error.instance_path);
                        println!();
                    }

                    panic!("Validation failed");
                }
            }
        }
    };
}

parse_and_analyse!(debugging);
parse_and_analyse!(gesture);
parse_and_analyse!(microspeech);
parse_and_analyse!(noop);
parse_and_analyse!(person_detection);
parse_and_analyse!(sine);
parse_and_analyse!(style_transfer);

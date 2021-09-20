use codespan_reporting::{
    files::SimpleFile,
    term::{termcolor::Buffer, Config},
};
use hotg_rune_syntax::{
    Diagnostics,
    parse::Document,
    hooks::{Hooks, AfterTypeCheckingContext, Continuation},
    BuildContext, Verbosity,
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

            #[derive(Default)]
            struct AbortAfterTypecheck {
                diags: Diagnostics,
            }

            impl Hooks for AbortAfterTypecheck {
                fn after_type_checking(
                    &mut self,
                    ctx: &mut dyn AfterTypeCheckingContext,
                ) -> Continuation {
                    for diag in ctx.diagnostics().iter() {
                        self.diags.push(diag.clone());
                    }

                    Continuation::Halt
                }
            }

            #[test]
            fn analyse() {
                let file = SimpleFile::new("Runefile", SRC);
                let ctx = BuildContext {
                    name: stringify!($example).to_string(),
                    runefile: SRC.to_string(),
                    working_directory: PATH.into(),
                    current_directory: PATH.into(),
                    optimized: false,
                    verbosity: Verbosity::Normal,
                };
                let mut hooks = AbortAfterTypecheck::default();

                hotg_rune_syntax::build_with_hooks(ctx, &mut hooks);

                let mut writer = Buffer::no_color();
                let config = Config::default();

                for diag in &hooks.diags {
                    codespan_reporting::term::emit(
                        &mut writer,
                        &config,
                        &file,
                        diag,
                    )
                    .unwrap();
                }

                if hooks.diags.has_errors() {
                    panic!("{}", String::from_utf8_lossy(writer.as_slice()));
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

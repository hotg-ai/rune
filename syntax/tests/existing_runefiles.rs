use codespan_reporting::{
    files::SimpleFiles,
    term::{
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
};
use rune_syntax::Diagnostics;

macro_rules! parse_and_analyse {
    ($example:ident) => {
        mod $example {
            use super::*;
            const SRC: &str = include_str!(concat!(
                "../../examples/",
                stringify!($example),
                "/Runefile"
            ));

            #[test]
            fn parse() {
                match rune_syntax::parse(SRC) {
                    Ok(parsed) => {
                        let expected_span =
                            codespan::Span::new(0, SRC.len() as u32);
                        assert_eq!(parsed.span, expected_span);
                    },
                    Err(e) => panic!("{}", e),
                }
            }

            #[test]
            fn analyse() {
                let mut files = SimpleFiles::new();
                let id = files.add("Runefile", SRC);

                let parsed = rune_syntax::parse(SRC).unwrap();

                assert!(parsed.instructions.len() > 1);

                let mut diags = Diagnostics::new();
                rune_syntax::analyse(id, &parsed, &mut diags);

                let mut writer = StandardStream::stdout(ColorChoice::Auto);
                let config = Config::default();

                for diag in &diags {
                    codespan_reporting::term::emit(
                        &mut writer,
                        &config,
                        &files,
                        diag,
                    )
                    .unwrap();
                }

                if !diags.is_empty() {
                    panic!("There were errors");
                }
            }
        }
    };
}

parse_and_analyse!(sine);
parse_and_analyse!(gesture);

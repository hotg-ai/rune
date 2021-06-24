use codespan_reporting::{
    files::SimpleFile,
    term::{termcolor::Buffer, Config},
};
use rune_syntax::Diagnostics;

macro_rules! parse_and_analyse {
    ($example:ident) => {
        mod $example {
            use super::*;
            const SRC: &str = include_str!(concat!(
                "../../../examples/",
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
                let file = SimpleFile::new("Runefile", SRC);

                let parsed = rune_syntax::parse(file.source()).unwrap();

                assert!(parsed.instructions.len() > 1);

                let mut diags = Diagnostics::new();
                rune_syntax::analyse(&parsed, &mut diags);

                let mut writer = Buffer::no_color();
                let config = Config::default();

                for diag in &diags {
                    codespan_reporting::term::emit(
                        &mut writer,
                        &config,
                        &file,
                        diag,
                    )
                    .unwrap();
                }

                if diags.has_errors() {
                    panic!("{}", String::from_utf8_lossy(writer.as_slice()));
                }
            }
        }
    };
}

parse_and_analyse!(sine);
parse_and_analyse!(gesture);
parse_and_analyse!(microspeech);

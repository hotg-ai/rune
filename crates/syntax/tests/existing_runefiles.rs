use codespan_reporting::{
    files::SimpleFile,
    term::{termcolor::Buffer, Config},
};
use hotg_rune_syntax::{Diagnostics, yaml::Document};

macro_rules! parse_and_analyse {
    ($example:ident) => {
        mod $example {
            use super::*;
            const SRC: &str = include_str!(concat!(
                "../../../examples/",
                stringify!($example),
                "/Runefile.yml"
            ));

            #[test]
            fn parse() { let _ = Document::parse(SRC).unwrap(); }

            #[test]
            fn analyse() {
                let file = SimpleFile::new("Runefile", SRC);

                let parsed = Document::parse(file.source()).unwrap();

                let mut diags = Diagnostics::new();
                hotg_rune_syntax::analyse(parsed, &mut diags);

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

parse_and_analyse!(debugging);
parse_and_analyse!(gesture);
parse_and_analyse!(microspeech);
parse_and_analyse!(noop);
parse_and_analyse!(person_detection);
parse_and_analyse!(sine);
parse_and_analyse!(style_transfer);

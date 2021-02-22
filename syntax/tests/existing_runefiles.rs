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
        #[test]
        fn $example() {
            let src = include_str!(concat!(
                "../../examples/",
                stringify!($example),
                "/Runefile"
            ));
            let mut files = SimpleFiles::new();
            let id = files.add("Runefile", src);

            let parsed = rune_syntax::parse(src).unwrap();

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
    };
}

parse_and_analyse!(sine);
// parse_runefile!(microspeech);

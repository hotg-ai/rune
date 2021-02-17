macro_rules! parse_runefile {
    ($example:ident) => {
        #[test]
        fn $example() {
            let src = include_str!(concat!(
                "../../examples/",
                stringify!($example),
                "/Runefile"
            ));

            let parsed = rune_syntax::parse(src).unwrap();

            assert!(parsed.instructions.len() > 1);
        }
    };
}

parse_runefile!(sine);
// parse_runefile!(microspeech);

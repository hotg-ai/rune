use std::str::FromStr;

use codespan::Span;
use pest::{error::Error, iterators::Pair, Parser, RuleType};

use crate::ast::{
    Argument, ArgumentValue, CapabilityInstruction, FromInstruction, Ident,
    Literal, LiteralKind, ModelInstruction, OutInstruction, Path,
    ProcBlockInstruction, RunInstruction, Runefile, Type, TypeKind,
};

/// Parse a [`Runefile`] from its textual representation.
pub fn parse(src: &str) -> Result<Runefile, Error<Rule>> {
    let span = Span::new(0, src.len() as u32);
    let parsed = RunefileParser::parse(Rule::runefile, src)?;

    let mut instructions = Vec::new();

    for statement in parsed.into_iter().next().unwrap().into_inner() {
        if statement.as_rule() == Rule::EOI {
            break;
        }

        debug_assert_eq!(statement.as_rule(), Rule::statement);
        let instruction = statement.into_inner().next().unwrap();

        match instruction.as_rule() {
            Rule::from => instructions.push(parse_from(instruction).into()),
            Rule::out => instructions.push(parse_out(instruction).into()),
            Rule::model => instructions.push(parse_model(instruction).into()),
            Rule::capability => {
                instructions.push(parse_capability(instruction).into())
            },
            Rule::run => instructions.push(parse_run(instruction).into()),
            Rule::proc_block => {
                instructions.push(parse_proc_block(instruction).into())
            },
            _ => unreachable!("{:?}", instruction),
        }
    }

    Ok(Runefile { instructions, span })
}

impl FromStr for Runefile {
    type Err = Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> { parse(s) }
}

#[derive(pest_derive::Parser)]
#[grammar = "runefile.pest"]
pub struct RunefileParser;

fn parse_ident(pair: Pair<Rule>) -> Ident {
    debug_assert_eq!(pair.as_rule(), Rule::ident);

    Ident {
        value: pair.as_str().to_string(),
        span: get_span(&pair),
    }
}

fn get_span<R: RuleType>(pair: &Pair<R>) -> Span {
    let s = pair.as_span();
    Span::new(s.start() as u32, s.end() as u32)
}

fn parse_type(pair: Pair<Rule>) -> Type {
    let span = get_span(&pair);

    debug_assert_eq!(pair.as_rule(), Rule::ty);
    let pair = pair.into_inner().next().unwrap();

    let kind = match pair.as_rule() {
        Rule::inferred_type => TypeKind::Inferred,
        Rule::type_with_dimensions => {
            let mut parts = pair.into_inner();
            let type_name = parse_ident(parts.next().unwrap());
            match parts.next() {
                Some(dimensions) => TypeKind::Buffer {
                    type_name,
                    dimensions: parse_dimensions(dimensions),
                },
                None => TypeKind::Named(type_name),
            }
        },
        _ => unreachable!("{:?}", pair),
    };

    Type { kind, span }
}

impl FromStr for Type {
    type Err = Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RunefileParser::parse(Rule::ty, s)
            .map(|mut pairs| parse_type(pairs.next().unwrap()))
    }
}

fn parse_dimensions(pair: Pair<Rule>) -> Vec<usize> {
    debug_assert_eq!(pair.as_rule(), Rule::dimensions);

    pair.into_inner()
        .map(|p| p.as_str().parse::<usize>().unwrap())
        .collect()
}

fn parse_literal(pair: Pair<Rule>) -> Literal {
    let span = get_span(&pair);

    let kind = match pair.as_rule() {
        Rule::integer => LiteralKind::Integer(pair.as_str().parse().unwrap()),
        Rule::float => LiteralKind::Float(pair.as_str().parse().unwrap()),
        Rule::string => LiteralKind::String(pair.as_str().to_string()),
        Rule::literal => {
            return parse_literal(pair.into_inner().next().unwrap())
        },
        _ => unreachable!("{:?}", pair),
    };

    Literal { kind, span }
}

impl FromStr for Literal {
    type Err = Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RunefileParser::parse(Rule::literal, s)
            .map(|mut pairs| parse_literal(pairs.next().unwrap()))
    }
}

fn parse_argument(pair: Pair<Rule>) -> Argument {
    let span = get_span(&pair);

    debug_assert_eq!(pair.as_rule(), Rule::argument);
    let mut pair = pair.into_inner();

    let name = parse_argument_name(pair.next().unwrap());

    let value = pair.next().unwrap();
    let value = match value.as_rule() {
        Rule::literal => ArgumentValue::Literal(parse_literal(value)),
        Rule::arg_list => ArgumentValue::List(
            value.into_inner().map(|p| p.as_str().to_string()).collect(),
        ),
        Rule::arg_list_item => ArgumentValue::Literal(Literal::new(
            value.as_str(),
            get_span(&value),
        )),
        _ => unreachable!("{:?}", value),
    };

    Argument { name, value, span }
}

fn parse_argument_name(pair: Pair<Rule>) -> Ident {
    let span = get_span(&pair);
    debug_assert_eq!(pair.as_rule(), Rule::arg_name);

    let value = pair.into_inner().next().unwrap().as_str().to_string();

    Ident { value, span }
}

fn parse_path(pair: Pair<Rule>) -> Path {
    let span = get_span(&pair);

    debug_assert_eq!(pair.as_rule(), Rule::path);
    let mut pair = pair.into_inner();

    let base = pair.next().unwrap().as_str().to_string();

    let (version, sub_path) = match (pair.next(), pair.next()) {
        (Some(version), Some(sub_path)) => (
            Some(version.as_str().to_string()),
            Some(sub_path.as_str().to_string()),
        ),
        (Some(unknown), None) if unknown.as_rule() == Rule::path_version => {
            (Some(unknown.as_str().to_string()), None)
        },
        (Some(unknown), None) if unknown.as_rule() == Rule::path_segments => {
            (None, Some(unknown.as_str().to_string()))
        },
        _ => (None, None),
    };

    Path {
        base,
        sub_path,
        version,
        span,
    }
}

impl FromStr for Path {
    type Err = Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RunefileParser::parse(Rule::path, s)
            .map(|mut pairs| parse_path(pairs.next().unwrap()))
    }
}

fn parse_run(pair: Pair<Rule>) -> RunInstruction {
    let span = get_span(&pair);
    debug_assert_eq!(pair.as_rule(), Rule::run);

    RunInstruction {
        steps: pair.into_inner().map(parse_ident).collect(),
        span,
    }
}

impl FromStr for RunInstruction {
    type Err = Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RunefileParser::parse(Rule::run, s)
            .map(|mut pairs| parse_run(pairs.next().unwrap()))
    }
}

fn parse_proc_block(pair: Pair<Rule>) -> ProcBlockInstruction {
    let span = get_span(&pair);

    debug_assert_eq!(pair.as_rule(), Rule::proc_block);
    let mut pair = pair.into_inner();

    let input_type = parse_type(pair.next().unwrap());
    let output_type = parse_type(pair.next().unwrap());
    let name = parse_ident(pair.next().unwrap());
    let path = parse_path(pair.next().unwrap());
    let args = parse_args(pair.next().unwrap());

    ProcBlockInstruction {
        name,
        path,
        input_type,
        output_type,
        params: args,
        span,
    }
}

impl FromStr for ProcBlockInstruction {
    type Err = Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RunefileParser::parse(Rule::proc_block, s)
            .map(|mut pairs| parse_proc_block(pairs.next().unwrap()))
    }
}

fn parse_args(pair: Pair<Rule>) -> Vec<Argument> {
    debug_assert_eq!(pair.as_rule(), Rule::arguments);

    pair.into_inner().map(parse_argument).collect()
}

fn parse_out(pair: Pair<Rule>) -> OutInstruction {
    let span = get_span(&pair);
    debug_assert_eq!(pair.as_rule(), Rule::out);

    let name = pair.into_inner().next().unwrap();

    OutInstruction {
        out_type: parse_ident(name),
        span,
    }
}

impl FromStr for OutInstruction {
    type Err = Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RunefileParser::parse(Rule::out, s)
            .map(|mut pairs| parse_out(pairs.next().unwrap()))
    }
}

fn parse_from(pair: Pair<Rule>) -> FromInstruction {
    let span = get_span(&pair);

    debug_assert_eq!(pair.as_rule(), Rule::from);
    let mut pair = pair.into_inner();

    let image = parse_path(pair.next().unwrap());

    FromInstruction { image, span }
}

impl FromStr for FromInstruction {
    type Err = Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RunefileParser::parse(Rule::from, s)
            .map(|mut pairs| parse_from(pairs.next().unwrap()))
    }
}

fn parse_capability(pair: Pair<Rule>) -> CapabilityInstruction {
    let span = get_span(&pair);

    debug_assert_eq!(pair.as_rule(), Rule::capability);
    let mut pair = pair.into_inner();

    let output_type = parse_type(pair.next().unwrap());
    let name = parse_ident(pair.next().unwrap());
    let kind = parse_ident(pair.next().unwrap());
    let parameters = parse_args(pair.next().unwrap());

    CapabilityInstruction {
        kind,
        name,
        output_type,
        parameters,
        span,
    }
}

impl FromStr for CapabilityInstruction {
    type Err = Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RunefileParser::parse(Rule::capability, s)
            .map(|mut pairs| parse_capability(pairs.next().unwrap()))
    }
}

fn parse_model(pair: Pair<Rule>) -> ModelInstruction {
    let span = get_span(&pair);

    debug_assert_eq!(pair.as_rule(), Rule::model);
    let mut pair = pair.into_inner();

    let input_type = parse_type(pair.next().unwrap());
    let output_type = parse_type(pair.next().unwrap());
    let name = parse_ident(pair.next().unwrap());
    let file = pair.next().unwrap().as_str().to_string();
    let parameters = parse_args(pair.next().unwrap());

    ModelInstruction {
        file,
        input_type,
        output_type,
        name,
        parameters,
        span,
    }
}

impl FromStr for ModelInstruction {
    type Err = Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RunefileParser::parse(Rule::model, s)
            .map(|mut pairs| parse_model(pairs.next().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        CapabilityInstruction, FromInstruction, ModelInstruction,
        OutInstruction, Type, TypeKind,
    };
    use pest::Parser;

    #[test]
    fn parse_a_from_instruction() {
        let src = "FROM runicos/base";
        let should_be = FromInstruction {
            image: Path::new("runicos/base", None, None, Span::new(5, 17)),
            span: Span::new(0, 17),
        };

        let got = RunefileParser::parse(Rule::from, src)
            .unwrap()
            .next()
            .unwrap();
        let got = parse_from(got);

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_a_capability() {
        let src = "CAPABILITY<I32> rand RAND --n 1";
        let should_be = CapabilityInstruction {
            name: Ident {
                value: String::from("rand"),
                span: Span::new(16, 20),
            },
            kind: Ident {
                value: String::from("RAND"),
                span: Span::new(21, 25),
            },
            output_type: Type {
                kind: TypeKind::Named(Ident {
                    value: String::from("I32"),
                    span: Span::new(11, 14),
                }),
                span: Span::new(11, 14),
            },
            parameters: vec![Argument::literal(
                Ident::new("n", Span::new(26, 29)),
                Literal::new(1, Span::new(30, 31)),
                Span::new(26, 31),
            )],
            span: Span::new(0, 31),
        };

        let got = RunefileParser::parse(Rule::capability, src)
            .unwrap()
            .next()
            .unwrap();
        let got = parse_capability(got);

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_a_model() {
        let src = "MODEL<_,_> sine ./sinemodel.tflite --input 1,1 --output 1,1";
        let should_be = ModelInstruction {
            name: Ident {
                value: String::from("sine"),
                span: Span::new(11, 15),
            },
            file: String::from("./sinemodel.tflite"),
            input_type: Type {
                kind: TypeKind::Inferred,
                span: Span::new(6, 7),
            },
            output_type: Type {
                kind: TypeKind::Inferred,
                span: Span::new(8, 9),
            },
            parameters: vec![
                Argument::list(
                    Ident::new("input", Span::new(35, 42)),
                    vec!["1", "1"],
                    Span::new(35, 46),
                ),
                Argument::list(
                    Ident::new("output", Span::new(47, 55)),
                    vec!["1", "1"],
                    Span::new(47, 59),
                ),
            ]
            .into_iter()
            .collect(),
            span: Span::new(0, 59),
        };

        let got = RunefileParser::parse(Rule::model, src)
            .unwrap()
            .next()
            .unwrap();
        let got = parse_model(got);

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_a_run_instruction() {
        let src = "RUN rand mod360 sine";
        let should_be = RunInstruction {
            steps: vec![
                Ident {
                    value: String::from("rand"),
                    span: Span::new(4, 8),
                },
                Ident {
                    value: String::from("mod360"),
                    span: Span::new(9, 15),
                },
                Ident {
                    value: String::from("sine"),
                    span: Span::new(16, 20),
                },
            ],
            span: Span::new(0, 20),
        };

        let got = RunefileParser::parse(Rule::run, src)
            .unwrap()
            .next()
            .unwrap();
        let got = parse_run(got);

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_an_out_instruction() {
        let src = "OUT serial";
        let should_be = OutInstruction {
            out_type: Ident {
                value: String::from("serial"),
                span: Span::new(4, 10),
            },
            span: Span::new(0, 10),
        };

        let got = RunefileParser::parse(Rule::out, src)
            .unwrap()
            .next()
            .unwrap();
        let got = parse_out(got);

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_a_proc_block() {
        let src = "PROC_BLOCK<_,_> mod360 hotg-ai/pb-mod@latest --modulo 100";
        let should_be = ProcBlockInstruction {
            path: Path::new(
                "hotg-ai/pb-mod",
                None,
                String::from("latest"),
                Span::new(23, 44),
            ),
            input_type: Type {
                kind: TypeKind::Inferred,
                span: Span::new(11, 12),
            },
            output_type: Type {
                kind: TypeKind::Inferred,
                span: Span::new(13, 14),
            },
            name: Ident {
                value: String::from("mod360"),
                span: Span::new(16, 22),
            },
            params: vec![Argument::literal(
                Ident::new("modulo", Span::new(45, 53)),
                Literal::new(100, Span::new(54, 57)),
                Span::new(45, 57),
            )]
            .into_iter()
            .collect(),
            span: Span::new(0, 57),
        };

        let got = RunefileParser::parse(Rule::proc_block, src)
            .unwrap()
            .next()
            .unwrap();
        let got = parse_proc_block(got);

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_path_to_file_inside_dependency() {
        let src = "hotg-ai/rune@v1.2#proc_blocks/normalize";
        let should_be = Path {
            base: String::from("hotg-ai/rune"),
            sub_path: Some(String::from("proc_blocks/normalize")),
            version: Some(String::from("v1.2")),
            span: Span::new(0, 39),
        };

        let got = RunefileParser::parse(Rule::path, src)
            .unwrap()
            .next()
            .unwrap();
        let got = parse_path(got);

        assert_eq!(got, should_be);
    }

    /// Assert that a set of strings parse successfully using the specified
    /// [`Rule`].
    macro_rules! assert_matches {
            ($test_name:ident for $rule:ident $(with $parser:ident)?, $($src:expr),* $(,)?) => {
                #[test]
                #[allow(non_snake_case)]
                fn $test_name() {
                    let input = vec![ $($src),* ];

                    for src in input {
                        let got = match RunefileParser::parse(Rule::$rule, src) {
                            Ok(got) => got,
                            Err(e) => panic!("{}\n\n{:?}", e, e),
                        };
                        assert_eq!( got.as_str().len(),
                            src.len(),
                            "Only parsed \"{}\" out of \"{}\"",
                            got.as_str(),
                            src,
                        );

                        $( let _ = $parser(got.clone().next().unwrap()); )?
                    }
                }
            };
            ($rule:ident $(with $parser:ident)?, $($src:expr),* $(,)?) => {
                assert_matches!($rule for $rule $(with $parser)?, $($src),*);
            };
        }

    macro_rules! assert_doesnt_match {
            ($test_name:ident for $rule:ident, $($src:expr),* $(,)?) => {
                #[test]
                #[allow(non_snake_case)]
                fn $test_name() {
                    $(
                        match RunefileParser::parse(Rule::$rule, $src) {
                            Ok(got) => assert_ne!(
                                got.as_str().len(),
                                $src.len(),
                                "Expected parsing \"{}\" to fail but got {:?}",
                                $src,
                                got,
                            ),
                            _ => {},
                        }
                    )*
                }
            };
        }

    assert_matches!(ident with parse_ident, "f", "fff", "f32", "F_12");
    assert_doesnt_match!(reject_invalid_idents for ident, "_", "a-b");
    assert_matches!(
        string with parse_literal,
        r#""""#,
        r#""asdf""#,
        r#""\x0a""#,
        r#""\uabcd""#,
        r#""\n""#,
        r#""\t""#,
        r#""\\""#,
        r#""\r""#,
    );
    assert_matches!(
        ty with parse_type,
        "u32",
        "uint32_t",
        "f32[1]",
        "f32[1, 150]",
        "UTF8",
        "_"
    );
    assert_doesnt_match!(invalid_types for ty, "f32[150][1]", "[f32; 5]", "u32[1 x 128]");
    assert_matches!(integer with parse_literal, "1", "0", "42", "-123");
    assert_matches!(float with parse_literal, "1.0", "1.", "-5.0", "1e2", "1e-2", "1.0e10");
    assert_doesnt_match!(invalid_floats for float, "1", "-42", ".5", "1e");
    assert_matches!(from, "FROM runicos/base", "FROM \\\n  runicos/base");
    assert_matches!(
        capability,
        "CAPABILITY<f32> rand RAND",
        "CAPABILITY<I32> rand RAND --n 1"
    );
    assert_matches!(arg_name, "-n", "--number", "--n", "--long-arg");
    assert_matches!(arg_value, "4", "3.14", "-123", r#""Hello, World!""#);
    assert_matches!(
        argument with parse_argument,
        "-n 42",
        "--number 42",
        "--number \\\n 42",
        "--number=42",
        "--float 3.14",
        "--unquoted_string hello",
        r#"--quoted_string "hello""#,
        "--labels=Wing,Ring,Slope,Unknown"
    );
    assert_matches!(
        path with parse_path,
        "asdf",
        "runicos/base",
        "runicos/base@0.1.2",
        "runicos/base@latest",
        "hotg-ai/rune#proc_blocks/normalize",
        "https://github.com/hotg-ai/rune",
        "https://github.com/hotg-ai/rune@2"
    );
    assert_doesnt_match!(invalid_paths for path,
        "as df",
        "runicos:latest",
    );
    assert_matches!(
        proc_block with parse_proc_block,
        "PROC_BLOCK<_,_> mod360 hotg-ai/pb-mod",
        "PROC_BLOCK<_,_> mod360 hotg-ai/pb-mod --modulo 100"
    );
    assert_matches!(run with parse_run, "RUN rand mod360 sine");
    assert_matches!(
        model with parse_model,
        "MODEL<_,_> sine ./sinemodel.tflite",
        "MODEL<_,_> sine ./sinemodel.tflite --input 1,1 --output 1,1",
    );
    assert_matches!(
        runefile,
        "# This is a comment\nFROM asdf\n\n#comment\nFROM xcvb"
    );
}

//! The parser.

use std::collections::HashMap;

use crate::ast::{
    CapabilityInstruction, FromInstruction, Ident, Instruction,
    ModelInstruction, OutInstruction, ProcBlockInstruction, RunInstruction,
    Runefile, Type,
};
use codespan::Span;
use pest::{error::Error, iterators::Pair, Parser, RuleType};

#[derive(pest_derive::Parser)]
#[grammar = "runefile.pest"]
pub struct RunefileParser;

/// Parse a [`Runefile`] from its textual representation.
pub fn parse(src: &str) -> Result<Runefile, Error<Rule>> {
    let top_level = RunefileParser::parse(Rule::runefile, src)?
        .next()
        .expect("There is always a runefile rule");
    let span = get_span(&top_level);

    let mut instructions: Vec<Instruction> = Vec::new();

    for pair in top_level.into_inner() {
        match pair.as_rule() {
            Rule::from => {
                instructions.push(parse_from(pair).into());
            },
            Rule::capability => {
                instructions.push(parse_capability(pair).into());
            },
            Rule::proc_line => {
                instructions.push(parse_proc_block(pair).into());
            },
            Rule::model => {
                instructions.push(parse_model(pair).into());
            },
            Rule::run => {
                instructions.push(parse_run(pair).into());
            },
            Rule::out => {
                instructions.push(parse_out(pair).into());
            },
            Rule::EOI => {},
            other => todo!("Haven't implemented {:?}\n\n{:?}", other, pair),
        }
    }

    Ok(Runefile { instructions, span })
}

fn get_span<R: RuleType>(pair: &Pair<R>) -> Span {
    let s = pair.as_span();
    Span::new(s.start() as u32, s.end() as u32)
}

fn parse_from(pair: Pair<Rule>) -> FromInstruction {
    let span = get_span(&pair);

    let image = pair.into_inner().next().unwrap();
    let image = parse_ident(image);

    FromInstruction { image, span }
}

fn parse_ident(pair: Pair<Rule>) -> Ident {
    Ident {
        value: pair.as_str().to_string(),
        span: get_span(&pair),
    }
}

fn parse_capability(pair: Pair<Rule>) -> CapabilityInstruction {
    let span = get_span(&pair);
    let mut pairs = pair.into_inner();

    // Note: Guaranteed by the grammar to not panic

    let (input_type, output_type) = parse_input_types(pairs.next().unwrap());

    let name = parse_ident(pairs.next().unwrap());
    let description = pairs.next().unwrap().as_str().to_string();

    let mut parameters = HashMap::new();

    for step in pairs {
        let mut pairs = step.into_inner().next().unwrap().into_inner();
        let variable = pairs.next().unwrap();
        let argument = pairs.next().unwrap();

        parameters.insert(
            variable.as_str().to_string(),
            argument.as_str().to_string(),
        );
    }

    return CapabilityInstruction {
        name,
        description,
        input_type,
        output_type,
        parameters,
        span,
    };
}

fn parse_input_types(pair: Pair<Rule>) -> (Type, Type) {
    let mut pairs = pair.into_inner();
    let input_type = parse_type(pairs.next().unwrap());
    let output_type = parse_type(pairs.next().unwrap());

    (input_type, output_type)
}

fn parse_type(pair: Pair<Rule>) -> Type {
    let span = get_span(&pair);

    if pair.as_str() == "_" {
        Type {
            kind: crate::ast::TypeKind::Inferred,
            span,
        }
    } else if pair.as_str().chars().all(char::is_alphanumeric) {
        let name = parse_ident(pair);
        Type {
            kind: crate::ast::TypeKind::Named(name),
            span,
        }
    } else {
        todo!("{:?}", pair)
    }
}

fn null_ident() -> Ident {
    Ident {
        value: String::new(),
        span: Span::new(0, 0),
    }
}

fn parse_proc_block(pair: Pair<Rule>) -> ProcBlockInstruction {
    let span = get_span(&pair);
    let mut parameters_param = HashMap::new();
    let mut path = String::new();
    let mut name = null_ident();

    for step_record in pair.into_inner() {
        match step_record.as_rule() {
            Rule::proc_path => path = step_record.as_str().to_string(),
            Rule::proc_name => name = parse_ident(step_record),
            Rule::proc_args => {
                for arg in step_record.into_inner() {
                    match arg.as_rule() {
                        Rule::proc_step => {
                            let mut last_param_name = String::new();

                            for part in arg.into_inner() {
                                match part.as_rule() {
                                    Rule::proc_arg_variable => {
                                        last_param_name =
                                            part.as_str().to_string();
                                    },
                                    Rule::proc_arg_value => {
                                        let last_param_value =
                                            part.as_str().to_string();
                                        let last_param_name_cloned =
                                            std::mem::take(
                                                &mut last_param_name,
                                            );
                                        parameters_param.insert(
                                            last_param_name_cloned,
                                            last_param_value,
                                        );
                                    },
                                    _ => {},
                                }
                            }
                        },
                        _ => {},
                    }
                }
            },
            _ => {},
        }
    }

    ProcBlockInstruction {
        path,
        name,
        params: parameters_param,
        span,
    }
}

fn parse_model(pair: Pair<Rule>) -> ModelInstruction {
    let span = get_span(&pair);
    let mut parameters_param = HashMap::new();
    let mut model_name_param = null_ident();
    let mut model_file_param = "".to_string();

    for args in pair.into_inner() {
        match args.as_rule() {
            Rule::model_file => model_file_param = args.as_str().to_string(),
            Rule::model_name => model_name_param = parse_ident(args),
            Rule::model_args => {
                for arg in args.into_inner() {
                    match arg.as_rule() {
                        Rule::model_step => {
                            let mut last_param_name = "".to_string();
                            for part in arg.into_inner() {
                                match part.as_rule() {
                                    Rule::model_arg_variable => {
                                        last_param_name =
                                            part.as_str().to_string();
                                    },
                                    Rule::model_arg_value => {
                                        let last_param_value =
                                            part.as_str().to_string();
                                        let last_param_name_cloned =
                                            last_param_name.clone();
                                        parameters_param.insert(
                                            last_param_name_cloned,
                                            last_param_value,
                                        );
                                    },
                                    _ => {},
                                }
                            }
                        },
                        _ => {},
                    }
                }
            },
            _ => {},
        }
    }

    ModelInstruction {
        name: model_name_param,
        file: model_file_param,
        parameters: parameters_param,
        span,
    }
}

fn parse_run(pair: Pair<Rule>) -> RunInstruction {
    let span = get_span(&pair);

    let steps = pair
        .into_inner()
        .next()
        .unwrap()
        .into_inner()
        .map(parse_ident)
        .collect();

    RunInstruction { steps, span }
}

fn parse_out(pair: Pair<Rule>) -> OutInstruction {
    let span = get_span(&pair);
    let out_type = parse_ident(pair.into_inner().next().unwrap());

    OutInstruction { span, out_type }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{CapabilityInstruction, Type, TypeKind};

    #[test]
    fn parse_a_from_instruction() {
        let src = "FROM runicos/base";
        let should_be = FromInstruction {
            image: Ident {
                value: String::from("runicos/base"),
                span: Span::new(5, src.len() as u32),
            },
            span: Span::new(0, src.len() as u32),
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
        let src = "CAPABILITY<_,I32> RAND rand --n 1";
        let should_be = CapabilityInstruction {
            name: Ident {
                value: String::from("RAND"),
                span: Span::new(18, 22),
            },
            description: String::from("rand"),
            parameters: vec![(String::from("n"), String::from("1"))]
                .into_iter()
                .collect(),
            input_type: Type {
                kind: TypeKind::Inferred,
                span: Span::new(11, 12),
            },
            output_type: Type {
                kind: TypeKind::Named(Ident {
                    value: String::from("I32"),
                    span: Span::new(13, 16),
                }),
                span: Span::new(13, 16),
            },
            span: Span::new(0, 33),
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
        let src =
            "MODEL<_,_> ./sinemodel.tflite sine --input [1,1] --output [1,1]";
        let should_be = ModelInstruction {
            name: Ident {
                value: String::from("sine"),
                span: Span::new(30, 34),
            },
            file: String::from("./sinemodel.tflite"),
            parameters: vec![
                (String::from("input"), String::from("[1,1]")),
                (String::from("output"), String::from("[1,1]")),
            ]
            .into_iter()
            .collect(),
            span: Span::new(0, 63),
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
        let src = "PROC_BLOCK<_,_> hotg-ai/pb-mod mod360 --modulo 100";
        let should_be = ProcBlockInstruction {
            path: String::from("hotg-ai/pb-mod"),
            name: Ident {
                value: String::from("mod360"),
                span: Span::new(31, 37),
            },
            params: vec![(String::from("modulo"), String::from("100"))]
                .into_iter()
                .collect(),
            span: Span::new(0, 50),
        };

        let got = RunefileParser::parse(Rule::proc_line, src)
            .unwrap()
            .next()
            .unwrap();
        let got = parse_proc_block(got);

        assert_eq!(got, should_be);
    }

    /// Assert that a set of strings parse successfully using the specified
    /// [`Rule`].
    macro_rules! assert_matches {
        ($rule:ident, $($src:expr),* $(,)?) => {
            #[test]
            #[allow(non_snake_case)]
            fn $rule() {
                $(
                    if let Err(e) = RunefileParser::parse(Rule::$rule, $src) {
                        panic!("{}\n\n{:?}", e, e);
                    }
                )*
            }
        };
    }

    assert_matches!(
        proc_step,
        "--identifier asdf",
        "--integer-literal 42",
        "--buffer-type i32[1, 2]",
        "--array [1, 2]"
    );

    assert_matches!(
        INPUT_TYPES,
        "<_,_>",
        "<I32, _>",
        "<_, F32[1,2]>",
        "<U64[1][2], _>"
    );
}

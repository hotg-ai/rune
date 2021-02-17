use std::collections::HashMap;

use crate::ast::{
    CapabilityInstruction, FromInstruction, Ident, Instruction,
    ModelInstruction, ProcBlockInstruction, Runefile, Type,
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
    // FIXME: This was all copied from the previous implementation. Instead of
    // creating temporary variables and expecting our loop to update them with
    // the real thing, we should take a much more declarative/functional
    // approach.
    //
    // It's more idiomatic and we are less likely to have undesireable results
    // that way.

    let mut capability_parameters_param: HashMap<String, String> =
        HashMap::new();
    let mut capability_name_param = null_ident();
    let mut capability_description_param = "".to_string();
    let mut input_type = Type {
        kind: crate::ast::TypeKind::Inferred,
        span: Span::new(0, 0),
    };
    let mut output_type = input_type.clone();
    let dependencies_map: HashMap<String, String> = HashMap::new();

    for args in pair.into_inner() {
        match args.as_rule() {
            Rule::INPUT_TYPES => {
                for arg in args.into_inner() {
                    match arg.as_rule() {
                        Rule::input_type => {
                            input_type = parse_type(arg);
                        },
                        Rule::output_type => {
                            output_type = parse_type(arg);
                        },
                        other => {
                            unreachable!(
                                "Should never get a {:?} rule\n\n{:?}",
                                other, arg
                            );
                        },
                    }
                }
            },
            Rule::capability_name => {
                capability_name_param = parse_ident(args);
            },
            Rule::capability_description => {
                capability_description_param = args.as_str().to_string()
            },
            Rule::capability_args => {
                for arg in args.into_inner() {
                    match arg.as_rule() {
                        Rule::capability_step => {
                            let mut last_param_name = "".to_string();
                            for part in arg.into_inner() {
                                match part.as_rule() {
                                    Rule::capability_arg_variable => {
                                        last_param_name =
                                            part.as_str().to_string();
                                    },
                                    Rule::capability_arg_value => {
                                        let last_param_value =
                                            part.as_str().to_string();
                                        let last_param_name_cloned =
                                            last_param_name.clone();
                                        capability_parameters_param.insert(
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

    CapabilityInstruction {
        capability_name: capability_name_param,
        capability_description: capability_description_param,
        capability_parameters: capability_parameters_param,
        dependencies: dependencies_map,
        input_type,
        output_type,
    }
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
        dependencies: HashMap::new(),
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
        model_name: model_name_param,
        model_file: model_file_param,
        model_parameters: parameters_param,
        span,
    }
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
            capability_name: Ident {
                value: String::from("RAND"),
                span: Span::new(18, 22),
            },
            capability_description: String::from("rand"),
            capability_parameters: vec![(String::from("n"), String::from("1"))]
                .into_iter()
                .collect(),
            dependencies: Default::default(),
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
        };

        let got = RunefileParser::parse(Rule::capability, src)
            .unwrap()
            .next()
            .unwrap();
        let got = parse_capability(got);

        assert_eq!(got, should_be);
    }
}

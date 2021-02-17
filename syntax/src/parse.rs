use crate::ast::{FromInstruction, Ident, Instruction, Runefile};
use codespan::Span;
use pest::{error::Error, iterators::Pair, Parser, RuleType};

#[derive(pest_derive::Parser)]
#[grammar = "runefile.pest"]
pub struct RunefileParser;

/// Parse a [`Runefile`] from its textual representation.
pub fn parse(src: &str) -> Result<Runefile, Error<Rule>> {
    let parsed = RunefileParser::parse(Rule::runefile, src)?;
    let span = Span::new(0, src.len() as u32);

    let mut instructions: Vec<Instruction> = Vec::new();

    for pair in parsed {
        match pair.as_rule() {
            Rule::from => {
                instructions.push(parse_from(pair).into());
            },
            _ => todo!(),
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

#[cfg(test)]
mod tests {
    use super::*;

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
}

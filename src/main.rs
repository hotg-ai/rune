
use structopt::StructOpt;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;
use std::fs;

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Parser;

#[derive(Debug,StructOpt)]
enum Rune {
    Build {
        #[structopt(parse(from_os_str))]
        file: std::path::PathBuf
    },
    Exec {
        #[structopt()]
        container: String,
    },
    Containers {
        #[structopt()]
        subcommand: String
    },
}

fn main() {
    let args = Rune::from_args();
    match args {
        Rune::Build { file } => {
            build(file);
        }
        Rune::Exec { container } => {
            println!("Exec {:?}", container);
        }
        Rune::Containers { subcommand } => {
            println!("Container {:?}", subcommand);
        }
    }
}

fn build(file: std::path::PathBuf) {
    println!("Building {:?}", file);
    let contents:&str = &fs::read_to_string(file)
        .expect("Something went wrong reading the file");
    parse(contents);
}

fn parse(file_string:&str) {
    let pairs = Parser::parse(Rule::ident_list, file_string).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
                Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
                _ => unreachable!()
            };
        }
    }
}
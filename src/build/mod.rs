
use pest::Parser;
use runefile_parser::parser::*;

use crate::cli;

pub fn build(opts: crate::cli::BuildOpts) {
    println!("{:?}", opts);

    let successful_parse = RunefileParser::parse(Rule::runefile, "FROM x")
    .expect("unsuccessful parse")
        .next().unwrap();
    println!("{:?}", successful_parse);
}
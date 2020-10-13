use pest::Parser;
use runefile_parser::parser::*;
use std::fmt;
use std::fs;
use std::path::Path;

use crate::cli;

pub fn build(opts: crate::cli::BuildOpts) {

    // TODO don't use unwrap for prod
    let fileloc = opts.file.to_str().unwrap();
    
    let contents = fs::read_to_string(fileloc)
        .expect("Failed to load file");
    runefile_parser::parser::parse(contents);
    execute();
}

fn execute() {
    println!("HI");
}

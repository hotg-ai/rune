

use structopt::StructOpt;

mod rune;

// This is needed for the parse trait from pest
// We should move this to the lib `runefile_parser`
use pest::Parser;
use runefile_parser::parser::*;

// Describe the commands for the CLI tool
//  Build,
//  Exec, 
//  Containers 
#[derive(Debug,StructOpt)]
enum Rune {
    Build {
        #[structopt(parse(from_os_str))]
        file: std::path::PathBuf
    },
    Run {
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
    println!("{:?}", args);
    rune::hello();
    
    let successful_parse = RunefileParser::parse(Rule::runefile, "FROM x")
        .expect("unsuccessful parse")
            .next().unwrap();
    println!("{:?}", successful_parse);
}

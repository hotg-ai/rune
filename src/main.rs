


mod build;
mod run;

use log;
use env_logger;


use clap::{App, Arg, SubCommand};

const VERSION: &str = "v0.0.2";

fn main() {
    let mut builder = env_logger::Builder::new();
    builder.filter_module("rune", log::LevelFilter::Info);
    builder.filter_module("rune::*", log::LevelFilter::Info);
    builder.init();
    log::info!("Rune {}", VERSION);
    // Process the cli command
    let matches = App::new("rune")
    .version(VERSION)
    .about("A self-sufficient runtime for TinyML Containers")
    .subcommands(vec![
        SubCommand::with_name("build")
            .arg(Arg::with_name("runefile")
                .value_name("FILE")
                .default_value("Runefile")
                .help("Runefile to build")
                .takes_value(true)),
        SubCommand::with_name("run")
                .arg(Arg::with_name("rune")
                    .value_name("FILE")
                    .default_value("sine.rune")
                    .help("*.rune file to run")
                    .takes_value(true))
    ]).get_matches();

    
    if let Some(matches) = matches.subcommand_matches("build") {
        match matches.value_of("runefile")  {
            Some(x) => build::build(x),
            _ => log::info!("No runefile provided")

        }
    } else if let Some(matches) = matches.subcommand_matches("run") {
        match matches.value_of("rune")  {
            Some(x) => run::run(x),
            _ => log::info!("No runefile provided")

        }
    }
    
}


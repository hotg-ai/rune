


mod build;
mod run;

use log;
use env_logger;

use env_logger::Env;


use clap::{App, Arg, SubCommand};

const VERSION: &str = "v0.0.1";

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
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


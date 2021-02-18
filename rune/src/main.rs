#![feature(allocator_api)]
mod build;
mod run;

use env_logger;
use log;

use clap::{App, Arg, SubCommand};

const VERSION: &str = "v0.0.2";

/// Rune CLI
///   Provides two CLI subcommands (run, build)
fn main() {
    /// Setting up environment logger that will only show logs
    /// for rune crates. We can change this with env variables.
    let mut builder = env_logger::Builder::new();
    builder.filter_module("rune", log::LevelFilter::Info);
    builder.filter_module("rune::*", log::LevelFilter::Info);
    builder.filter_module("runic_os::rune", log::LevelFilter::Info);
    builder.init();

    // Log the version
    log::info!("Rune {}", VERSION);
    // Process the cli command
    let matches = App::new("rune")
        .version(VERSION)
        .about("A self-sufficient runtime for TinyML Containers")
        .subcommands(vec![
            SubCommand::with_name("build").arg(
                Arg::with_name("runefile")
                    .value_name("FILE")
                    .default_value("Runefile")
                    .help("Runefile to build")
                    .takes_value(true),
            ),
            SubCommand::with_name("run")
                .arg(
                    Arg::with_name("rune")
                        .value_name("FILE")
                        .default_value("sine.rune")
                        .help("*.rune file to run")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("number_of_runs")
                        .short("n")
                        .long("run_count")
                        .value_name("number_of_runs")
                        .default_value("1")
                        .help("Number of time to call the rune"),
                ),
        ])
        .get_matches();

    // If the subcommand matches `build`
    if let Some(matches) = matches.subcommand_matches("build") {
        match matches.value_of("runefile") {
            Some(x) => build::build(x),
            _ => log::info!("No runefile provided"),
        }
    } else if let Some(matches) = matches.subcommand_matches("run") {
        let rune = match matches.value_of("rune") {
            Some(x) => x,
            _ => {
                log::warn!("No Rune provided");
                std::process::exit(1);
            },
        };

        let number_of_runs = matches.value_of("number_of_runs").unwrap_or("10");

        let number_of_runs: i32 = match number_of_runs.parse::<i32>() {
            Ok(n) => n,
            Err(_err) => {
                log::warn!("Invalid number of runs: '{}'", number_of_runs);
                -1
            },
        };

        run::run(rune, number_of_runs);
    }
}

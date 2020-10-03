


use structopt::StructOpt;
mod rune;

// We should move this to the lib `runefile_parser`



mod cli;
mod build;

fn main() {

    // Process the cli command
    let opt = cli::Opts::from_args();
    handle_subcommand(opt);
    
}


fn handle_subcommand(opt: cli::Opts){
  
    if let Some(subcommand) = opt.commands{
        match subcommand {
            cli::Rune::Build(cfg) => {

                build::build(cfg);
            },
            cli::Rune::Containers(cfg) => {
                println!("Containers {:?}", cfg);
            },
            cli::Rune::Run(cfg) => {
                println!("Run {:?}", cfg);
            },

        }
    }
}

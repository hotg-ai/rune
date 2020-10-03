


use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "opts", about="Options for rune cli")]
pub struct Opts{

    #[structopt(short = "v",  parse(from_occurrences))]
    verbosity: u8,

    // SUBCOMMANDS
    #[structopt(subcommand)]
    pub commands: Option<Rune>

}

#[derive(Debug,StructOpt)]
#[structopt(name = "rune", about = "Containers for TinyML")]
pub enum Rune {
    #[structopt(name = "build")]
    Build(BuildOpts),
    #[structopt(name = "run")]
    Run(RunOpts),
    #[structopt(name = "container")]
    Containers(ContainerOpts),
}

#[derive(StructOpt, Debug)]
pub  struct BuildOpts {
    #[structopt(parse(from_os_str), default_value="Runefile")]
    pub file: std::path::PathBuf
}

#[derive(StructOpt, Debug)]
pub  struct RunOpts {
    #[structopt(parse(from_os_str))]
    pub file: std::path::PathBuf
}

#[derive(StructOpt, Debug)]
pub struct ContainerOpts {
    #[structopt(parse(from_os_str))]
    pub  file: std::path::PathBuf
}




use structopt::StructOpt;

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
    println!("{:?}", args);
}

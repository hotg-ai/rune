

use structopt::StructOpt;

#[derive(Debug, StructOpt)]

struct Cli {
    command: String,
}

fn main() {


    let args = Cli::from_args();
    println!("{:?}", args);
    
}

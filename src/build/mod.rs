
use log;
use std::fs;



pub fn build(fileloc: &str) {

    
    let contents = fs::read_to_string(fileloc);

    let contents = match contents {
        Ok(c) => c,
        Err(_err) => {
            log::error!("Failed to load file '{}'", fileloc);
            return 
        }
    };

    runefile_parser::parser::generate(contents);
    execute();
}

fn execute() {
    println!("HI");
}

//! An example program that just prints out the JSON schema for the Runefile
//! format.

use hotg_rune_compiler::parse::Document;

fn main() {
    let schema = schemars::schema_for!(Document);
    let json = serde_json::to_string_pretty(&schema).unwrap();

    println!("{}", json);
}

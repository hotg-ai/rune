use std::error::Error;
use hotg_rune_compiler::parse::Document;
use jsonschema::JSONSchema;
use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args()
        .nth(1)
        .expect("Usage: validate-runefile-schema <runefile>");

    let src = std::fs::read_to_string(&filename)?;
    let document: Value = serde_yaml::from_str(&src)?;

    let schema = schemars::schema_for!(Document);
    let schema = serde_json::to_value(&schema)?;

    let compiled_schema = JSONSchema::options()
        .compile(&schema)
        .map_err(|e| e.to_string())?;

    let result = compiled_schema.validate(&document);

    match result {
        Ok(_) => {
            println!("The Runefile is valid ✓");
            Ok(())
        },
        Err(errors) => {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
                println!();
            }

            Err("Validation failed ✗".into())
        },
    }
}

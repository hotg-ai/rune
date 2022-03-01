use anyhow::{Context, Error};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{Map, Value};

use crate::{runtime::OutputTensor, Tensor, TensorElement};

pub(crate) fn parse_serial(data: &[u8]) -> Result<Vec<OutputTensor>, Error> {
    if let Ok(s) = std::str::from_utf8(data) {
        log::trace!("Parsing serial output: {}", s);
    }

    let deserialized: OneOrMany = serde_json::from_slice(data)
        .context("Deserializing from JSON failed")?;

    let values = match deserialized {
        OneOrMany::Many(many) => many,
        OneOrMany::One(one) => vec![one],
    };

    let mut outputs = Vec::new();

    for value in values {
        let deserialized = deserialize_serial_tensor(value)?;
        outputs.push(deserialized);
    }

    Ok(outputs)
}

fn deserialize_serial_tensor(
    value: Map<String, Value>,
) -> Result<OutputTensor, Error> {
    match value.get("type_name").and_then(|v| v.as_str()) {
        Some("utf8") => deserialize_strings(value),
        Some("u8") => deserialize_numeric::<u8>(value),
        Some("i8") => deserialize_numeric::<i8>(value),
        Some("u16") => deserialize_numeric::<u16>(value),
        Some("i16") => deserialize_numeric::<i16>(value),
        Some("u32") => deserialize_numeric::<u32>(value),
        Some("i32") => deserialize_numeric::<i32>(value),
        Some("f32") => deserialize_numeric::<f32>(value),
        Some("u64") => deserialize_numeric::<u64>(value),
        Some("i64") => deserialize_numeric::<i64>(value),
        Some("f64") => deserialize_numeric::<f64>(value),
        Some(other) => anyhow::bail!("Unknown element type, {}", other),
        None => Err(Error::msg("The tensor didn't specify its element type")),
    }
}

fn deserialize_strings(
    object: Map<String, Value>,
) -> Result<OutputTensor, Error> {
    #[derive(Deserialize)]
    struct StringTensor {
        dimensions: Vec<usize>,
        elements: Vec<String>,
    }

    let value = Value::Object(object);
    let StringTensor {
        dimensions,
        elements,
    } = serde_json::from_value(value)?;

    Ok(OutputTensor::StringTensor {
        dimensions,
        strings: elements,
    })
}

fn deserialize_numeric<T>(
    object: Map<String, Value>,
) -> Result<OutputTensor, Error>
where
    T: TensorElement + DeserializeOwned,
{
    #[derive(Deserialize)]
    struct NumericTensor<T> {
        dimensions: Vec<usize>,
        elements: Vec<T>,
    }

    let value = Value::Object(object);
    let NumericTensor {
        dimensions,
        elements,
    }: NumericTensor<T> = serde_json::from_value(value)?;
    let tensor = Tensor::new(&elements, &dimensions);

    Ok(tensor.into())
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum OneOrMany {
    Many(Vec<Map<String, Value>>),
    One(Map<String, Value>),
}

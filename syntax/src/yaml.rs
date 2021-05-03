use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Document {
    pub image: String,
    pub pipeline: BTreeMap<String, Stage>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum Stage {
    Model {
        model: String,
        #[serde(default)]
        inputs: Vec<String>,
        #[serde(default)]
        outputs: Vec<Type>,
    },
    ProcBlock {
        #[serde(rename = "proc-block")]
        proc_block: String,
        #[serde(default)]
        inputs: Vec<String>,
        #[serde(default)]
        outputs: Vec<Type>,
        #[serde(default)]
        args: BTreeMap<String, Value>,
    },
    Capability {
        capability: String,
        #[serde(default)]
        outputs: Vec<Type>,
        #[serde(default)]
        args: BTreeMap<String, Value>,
    },
    Out {
        out: String,
        #[serde(default)]
        inputs: Vec<String>,
        #[serde(default)]
        args: BTreeMap<String, Value>,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Type {
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(default)]
    pub dimensions: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "kebab-case", untagged)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<Value>),
}

impl From<f64> for Value {
    fn from(f: f64) -> Value { Value::Float(f) }
}

impl From<i64> for Value {
    fn from(i: i64) -> Value { Value::Int(i) }
}

impl From<String> for Value {
    fn from(s: String) -> Value { Value::String(s) }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Value { Value::String(s.to_string()) }
}

impl From<Vec<Value>> for Value {
    fn from(list: Vec<Value>) -> Value { Value::List(list) }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! map {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$(($k, $v),)*]))
    };
    // set-like
    ($($v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$($v,)*]))
    };
}

    #[test]
    fn parse_yaml_pipeline() {
        let src = r#"
image: "runicos/base"

pipeline:
  audio:
    capability: SOUND
    outputs:
    - type: i16
      dimensions: [16000]
    args:
      hz: 16000

  fft:
    proc-block: "hotg-ai/rune#proc_blocks/fft"
    inputs:
    - audio
    outputs:
    - type: i8
      dimensions: [1960]

  model:
    model: "./model.tflite"
    inputs:
    - fft
    outputs:
    - type: i8
      dimensions: [6]

  label:
    proc-block: "hotg-ai/rune#proc_blocks/ohv_label"
    inputs:
    - model
    outputs:
    - type: utf8
    args:
      labels: ["silence", "unknown", "up", "down", "left", "right"]

  output:
    out: SERIAL
    inputs:
    - label
        "#;
        let should_be = Document {
            image: String::from("runicos/base"),
            pipeline: map! {
                "audio".into() => Stage::Capability {
                    capability: String::from("SOUND"),
                    outputs: vec![Type {
                        ty: String::from("i16"),
                        dimensions: vec![16000],
                    }],
                    args: map! { "hz".to_string() => Value::Int(16000) },
                },
                "output".into() => Stage::Out {
                    out: String::from("SERIAL"),
                    args: BTreeMap::new(),
                    inputs: vec![String::from("label")],
                },
                "label".into() => Stage::ProcBlock {
                    proc_block: String::from("hotg-ai/rune#proc_blocks/ohv_label"),
                    inputs: vec![String::from("model")],
                    outputs: vec![Type { ty: String::from("utf8"), dimensions: Vec::new() }],
                    args: map! {
                        String::from("labels") => Value::from(vec![
                            Value::from("silence"),
                            Value::from("unknown"),
                            Value::from("up"),
                            Value::from("down"),
                            Value::from("left"),
                            Value::from("right"),
                        ]),
                    },
                },
                "fft".into() => Stage::ProcBlock {
                    proc_block: String::from("hotg-ai/rune#proc_blocks/fft"),
                    inputs: vec![String::from("audio")],
                    outputs: vec![Type { ty: String::from("i8"), dimensions: vec![1960] }],
                    args: BTreeMap::new(),
                },
                "model".into() => Stage::Model {
                    model: String::from("./model.tflite"),
                    inputs: vec![String::from("fft")],
                    outputs: vec![Type { ty: String::from("i8"), dimensions: vec![6] }],
                },
            },
        };

        let got: Document = serde_yaml::from_str(src).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_audio_block() {
        let src = r#"
              capability: SOUND
              outputs:
              - type: i16
                dimensions: [16000]
              args:
                hz: 16000
        "#;
        let should_be = Stage::Capability {
            capability: String::from("SOUND"),
            outputs: vec![Type {
                ty: String::from("i16"),
                dimensions: vec![16000],
            }],
            args: map! { "hz".to_string() => Value::Int(16000) },
        };

        let got: Stage = serde_yaml::from_str(src).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_values() {
        let inputs = vec![
            ("42", Value::Int(42)),
            ("3.14", Value::Float(3.14)),
            ("\"42\"", Value::String(String::from("42"))),
            (
                "[1, 2.0, \"asdf\"]",
                Value::List(vec![
                    Value::Int(1),
                    Value::Float(2.0),
                    Value::String(String::from("asdf")),
                ]),
            ),
        ];

        for (src, should_be) in inputs {
            let got: Value = serde_yaml::from_str(src).unwrap();
            assert_eq!(got, should_be);
        }
    }
}

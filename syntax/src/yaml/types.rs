use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{self, Formatter, Display},
    str::FromStr,
};
use regex::Regex;
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
use codespan::Span;
use crate::{
    ast::{ArgumentValue, Literal},
    hir,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Document {
    pub image: Path,
    pub pipeline: HashMap<String, Stage>,
}

impl Document {
    pub fn parse(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}

impl FromStr for Document {
    type Err = serde_yaml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Document::parse(s) }
}

/// A specification for finding a dependency.
///
/// The full syntax is `base@version#sub_path` where
///
/// - `base` is a URL or the name of a repository on GitHub (e.g. `hotg-ai/rune`
///   or `https://github.com/hotg-ai/rune`)
/// - `version` is an optional field specifying the version (e.g. as a git tag)
/// - `sub_path` is an optional field which is useful when pointing to
///   repositories with multiple relevant items because it lets you specify
///   which directory the specified item is in.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Path {
    pub base: String,
    pub sub_path: Option<String>,
    pub version: Option<String>,
}

impl Path {
    pub fn new(
        base: impl Into<String>,
        sub_path: impl Into<Option<String>>,
        version: impl Into<Option<String>>,
    ) -> Self {
        Path {
            base: base.into(),
            sub_path: sub_path.into(),
            version: version.into(),
        }
    }
}

impl<'a> From<&'a Path> for crate::ast::Path {
    fn from(p: &'a Path) -> crate::ast::Path {
        let Path {
            base,
            sub_path,
            version,
        } = p;
        crate::ast::Path::new(
            base.clone(),
            sub_path.clone(),
            version.clone(),
            Span::new(0, 0),
        )
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Path {
            base,
            sub_path,
            version,
        } = self;

        write!(f, "{}", base)?;
        if let Some(sub) = sub_path {
            write!(f, "#{}", sub)?;
        }
        if let Some(version) = version {
            write!(f, "@{}", version)?;
        }

        Ok(())
    }
}

impl FromStr for Path {
    type Err = PathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"(?x)
        (?P<base>[\w\d:/_.-]+)
        (?:@(?P<version>[\w\d./-]+))?
        (?:\#(?P<sub_path>[\w\d._/-]+))?
        ",
            )
            .unwrap()
        });

        let captures = PATTERN.captures(s).ok_or(PathParseError)?;

        let base = captures["base"].to_string();
        let version = captures.name("version").map(|m| m.as_str().to_string());
        let sub_path =
            captures.name("sub_path").map(|m| m.as_str().to_string());

        Ok(Path {
            base,
            version,
            sub_path,
        })
    }
}

impl Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Cow::<'de, str>::deserialize(deserializer)?;

        s.parse().map_err(D::Error::custom)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct PathParseError;

impl Display for PathParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse the path")
    }
}

impl std::error::Error for PathParseError {}

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
        proc_block: Path,
        #[serde(default)]
        inputs: Vec<String>,
        #[serde(default)]
        outputs: Vec<Type>,
        #[serde(default)]
        args: HashMap<String, Value>,
    },
    Capability {
        capability: String,
        #[serde(default)]
        outputs: Vec<Type>,
        #[serde(default)]
        args: HashMap<String, Value>,
    },
    Out {
        out: String,
        #[serde(default)]
        inputs: Vec<String>,
        #[serde(default)]
        args: HashMap<String, Value>,
    },
}

impl Stage {
    pub fn inputs(&self) -> &[String] {
        match self {
            Stage::Model { inputs, .. }
            | Stage::ProcBlock { inputs, .. }
            | Stage::Out { inputs, .. } => inputs,
            Stage::Capability { .. } => &[],
        }
    }

    pub fn inputs_mut(&mut self) -> Option<&mut Vec<String>> {
        match self {
            Stage::Model { inputs, .. }
            | Stage::ProcBlock { inputs, .. }
            | Stage::Out { inputs, .. } => Some(inputs),
            Stage::Capability { .. } => None,
        }
    }

    pub fn output_type(&self) -> Option<&Type> {
        match self.output_types() {
            [] => None,
            [output] => Some(output),
            _ => unimplemented!("Multiple outputs aren't supported yet"),
        }
    }

    pub fn output_types(&self) -> &[Type] {
        match self {
            Stage::Model { outputs, .. }
            | Stage::ProcBlock { outputs, .. }
            | Stage::Capability { outputs, .. } => outputs,
            Stage::Out { .. } => &[],
        }
    }
}

impl<'a> From<&'a Stage> for hir::Stage {
    fn from(s: &'a Stage) -> hir::Stage {
        match s {
            Stage::Model { model, .. } => hir::Stage::Model(hir::Model {
                model_file: model.into(),
            }),
            Stage::ProcBlock {
                proc_block, args, ..
            } => hir::Stage::ProcBlock(hir::ProcBlock {
                path: proc_block.into(),
                parameters: to_parameters(args),
            }),
            Stage::Capability {
                capability, args, ..
            } => hir::Stage::Source(hir::Source {
                kind: capability.as_str().into(),
                parameters: to_parameters(args),
            }),
            Stage::Out { out, .. } => hir::Stage::Sink(hir::Sink {
                kind: out.as_str().into(),
            }),
        }
    }
}

fn to_parameters(
    yaml: &HashMap<String, Value>,
) -> HashMap<String, ArgumentValue> {
    let mut map = HashMap::new();

    for (key, value) in yaml {
        map.insert(key.clone(), value.clone().into());
    }

    map
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Type {
    #[serde(rename = "type")]
    pub name: String,
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

impl From<Value> for ArgumentValue {
    fn from(v: Value) -> ArgumentValue {
        match v {
            Value::Int(i) => {
                ArgumentValue::Literal(Literal::new(i, Span::new(0, 0)))
            },
            Value::Float(f) => {
                ArgumentValue::Literal(Literal::new(f, Span::new(0, 0)))
            },
            Value::String(s) => {
                ArgumentValue::Literal(Literal::new(s, Span::new(0, 0)))
            },
            Value::List(list) => {
                let mut items = Vec::new();
                for item in list {
                    if let Value::String(s) = item {
                        items.push(s.clone());
                    } else {
                        unimplemented!();
                    }
                }

                ArgumentValue::List(items)
            },
        }
    }
}

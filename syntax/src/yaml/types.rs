use std::{
    borrow::Cow,
    fmt::{self, Formatter, Display},
    str::FromStr,
};
use indexmap::IndexMap;
use regex::Regex;
use once_cell::sync::Lazy;
use serde::{
    de::{Error as _, Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};
use codespan::Span;
use crate::{
    ast::{ArgumentValue, Literal},
    hir,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Document {
    pub image: Path,
    pub pipeline: IndexMap<String, Stage>,
}

impl Document {
    pub fn parse(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    pub fn write_as_yaml<W>(&self, writer: W) -> Result<(), serde_yaml::Error>
    where
        W: std::io::Write,
    {
        serde_yaml::to_writer(writer, self)?;
        Ok(())
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

impl<'a> From<&'a crate::ast::Path> for Path {
    fn from(p: &'a crate::ast::Path) -> Path {
        let crate::ast::Path {
            base,
            sub_path,
            version,
            ..
        } = p;

        Path::new(base.clone(), sub_path.clone(), version.clone())
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
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        inputs: Vec<Input>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        outputs: Vec<Type>,
    },
    ProcBlock {
        #[serde(rename = "proc-block")]
        proc_block: Path,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        inputs: Vec<Input>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        outputs: Vec<Type>,
        #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
        args: IndexMap<String, Value>,
    },
    Capability {
        capability: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        outputs: Vec<Type>,
        #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
        args: IndexMap<String, Value>,
    },
    Out {
        out: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        inputs: Vec<Input>,
        #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
        args: IndexMap<String, Value>,
    },
}

impl Stage {
    pub fn inputs(&self) -> &[Input] {
        match self {
            Stage::Model { inputs, .. }
            | Stage::ProcBlock { inputs, .. }
            | Stage::Out { inputs, .. } => inputs,
            Stage::Capability { .. } => &[],
        }
    }

    pub fn inputs_mut(&mut self) -> Option<&mut Vec<Input>> {
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

    pub fn span(&self) -> Span {
        // TODO: Get span from serde_yaml
        Span::new(0, 0)
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
                parameters: args
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k.replace("-", "_"), v))
                    .collect(),
            }),
            Stage::Capability {
                capability, args, ..
            } => hir::Stage::Source(hir::Source {
                kind: capability.as_str().into(),
                parameters: args
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k.replace("-", "_"), v))
                    .collect(),
            }),
            Stage::Out { out, .. } => hir::Stage::Sink(hir::Sink {
                kind: out.as_str().into(),
            }),
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Type {
    #[serde(rename = "type")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "kebab-case", untagged)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    List(Vec<Value>),
}

impl From<f32> for Value {
    fn from(f: f32) -> Value { Value::Float(f) }
}

impl From<i32> for Value {
    fn from(i: i32) -> Value { Value::Int(i) }
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
                ArgumentValue::Literal(Literal::new(i as i64, Span::new(0, 0)))
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

#[derive(Debug, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct Input {
    pub name: String,
    pub index: Option<usize>,
}

impl Input {
    pub fn new(
        name: impl Into<String>,
        index: impl Into<Option<usize>>,
    ) -> Self {
        Input {
            name: name.into(),
            index: index.into(),
        }
    }
}

impl FromStr for Input {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(?P<name>[a-zA-Z_][\w-]*)(?:\.(?P<index>\d+))?$")
                .unwrap()
        });

        let captures = PATTERN
            .captures(s)
            .ok_or("Expected something like \"fft\" or \"fft.2\"")?;

        let name = &captures["name"];
        let index = captures.name("index").map(|m| {
            m.as_str()
                .parse::<usize>()
                .expect("Guaranteed by the regex")
        });

        Ok(Input::new(name, index))
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.index {
            Some(index) => write!(f, "{}.{}", self.name, index),
            None => write!(f, "{}", self.name),
        }
    }
}

impl Serialize for Input {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Input {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = Cow::<str>::deserialize(deserializer)?;
        Input::from_str(&raw).map_err(|e| D::Error::custom(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_normal_input_specifier() {
        let src = "audio";
        let should_be = Input::new("audio", None);

        let got = Input::from_str(src).unwrap();

        assert_eq!(got, should_be);
        assert_eq!(got.to_string(), src);
    }

    #[test]
    fn input_specifier_with_tuple() {
        let src = "audio.2";
        let should_be = Input::new("audio", 2);

        let got = Input::from_str(src).unwrap();

        assert_eq!(got, should_be);
        assert_eq!(got.to_string(), src);
    }

    #[test]
    fn parse_paths() {
        let inputs = vec![
            ("asdf", Path::new("asdf", None, None)),
            ("runicos/base", Path::new("runicos/base", None, None)),
            (
                "runicos/base@0.1.2",
                Path::new("runicos/base", None, Some(String::from("0.1.2"))),
            ),
            (
                "runicos/base@latest",
                Path::new("runicos/base", None, Some(String::from("latest"))),
            ),
            (
                "hotg-ai/rune#proc_blocks/normalize",
                Path::new(
                    "hotg-ai/rune",
                    Some(String::from("proc_blocks/normalize")),
                    None,
                ),
            ),
            (
                "https://github.com/hotg-ai/rune",
                Path::new("https://github.com/hotg-ai/rune", None, None),
            ),
            (
                "https://github.com/hotg-ai/rune@2",
                Path::new(
                    "https://github.com/hotg-ai/rune",
                    None,
                    Some(String::from("2")),
                ),
            ),
        ];

        for (src, should_be) in inputs {
            let got: Path = src.parse().unwrap();
            assert_eq!(got, should_be, "{}", src);
        }
    }
}

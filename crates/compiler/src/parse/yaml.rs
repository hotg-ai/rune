//! Definitions for the Runefile's YAML format.

use std::{
    borrow::Cow,
    fmt::{self, Formatter, Display},
    ops::Deref,
    str::FromStr,
};
use schemars::{JsonSchema, schema::Schema, gen::SchemaGenerator};
use indexmap::IndexMap;
use regex::Regex;
use once_cell::sync::Lazy;
use serde::{
    de::{Error as _, Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};
use codespan::Span;

static RESOURCE_NAME_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\$[_a-zA-Z][_a-zA-Z0-9]*$").unwrap());

#[derive(Debug, Clone, PartialEq, JsonSchema)]
pub enum Document {
    V1(DocumentV1),
}

impl Document {
    pub fn to_v1(self) -> DocumentV1 {
        match self {
            Document::V1(d) => d,
        }
    }
}

impl From<DocumentV1> for Document {
    fn from(v1: DocumentV1) -> Self { Document::V1(v1) }
}

mod document_serde {
    use serde::de::Unexpected;
    use serde_yaml::Value;

    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Repr<T> {
        version: usize,
        #[serde(flatten)]
        inner: T,
    }

    impl<T> Repr<T> {
        fn new(version: usize, inner: T) -> Self { Repr { version, inner } }
    }

    impl Serialize for Document {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Document::V1(v1) => Repr::new(1, v1).serialize(serializer),
            }
        }
    }

    impl<'de> Deserialize<'de> for Document {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value = Value::deserialize(deserializer)?;
            let version_key = Value::from("version");
            let version = value
                .as_mapping()
                .and_then(|m| m.get(&version_key))
                .and_then(|v| v.as_u64());

            match version {
                Some(1) => {
                    let v1: DocumentV1 = serde_yaml::from_value(value)
                        .map_err(D::Error::custom)?;
                    Ok(Document::V1(v1))
                },
                Some(other) => Err(D::Error::invalid_value(
                    Unexpected::Unsigned(other),
                    &"version to be 1",
                )),
                None => Err(D::Error::missing_field("version")),
            }
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct DocumentV1 {
    pub image: Image,
    pub pipeline: IndexMap<String, Stage>,
    #[serde(default)]
    pub resources: IndexMap<String, ResourceDeclaration>,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, JsonSchema)]
#[schemars(with = "String")]
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

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum Stage {
    Model {
        model: ResourceOrString,
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
        Span::default()
    }
}

/// Something that could be either a reference to a resource (`$resource`)
/// or a plain string (`./path`).
#[derive(Debug, Clone, PartialEq, JsonSchema)]
#[schemars(with = "String")]
pub enum ResourceOrString {
    Resource(ResourceName),
    String(String),
}

impl Serialize for ResourceOrString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ResourceOrString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let repr = Cow::<str>::deserialize(deserializer)?;

        if repr.starts_with("$") {
            match ResourceName::from_str(&repr) {
                Ok(name) => Ok(ResourceOrString::Resource(name)),
                Err(e) => Err(D::Error::custom(e)),
            }
        } else {
            Ok(ResourceOrString::String(repr.into_owned()))
        }
    }
}

impl Display for ResourceOrString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ResourceOrString::String(path) => write!(f, "{}", path),
            ResourceOrString::Resource(res) => write!(f, "{}", res),
        }
    }
}

impl<S: Into<String>> From<S> for ResourceOrString {
    fn from(s: S) -> Self { ResourceOrString::String(s.into()) }
}

impl From<ResourceName> for ResourceOrString {
    fn from(name: ResourceName) -> Self { ResourceOrString::Resource(name) }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct Type {
    #[serde(rename = "type")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<usize>,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename = "kebab-case", untagged)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(ResourceOrString),
    List(Vec<Value>),
}

impl From<f32> for Value {
    fn from(f: f32) -> Value { Value::Float(f) }
}

impl From<i32> for Value {
    fn from(i: i32) -> Value { Value::Int(i) }
}

impl From<String> for Value {
    fn from(s: String) -> Value { Value::String(s.into()) }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Value { Value::String(s.into()) }
}

impl From<ResourceName> for Value {
    fn from(name: ResourceName) -> Value { Value::String(name.into()) }
}

impl From<Vec<Value>> for Value {
    fn from(list: Vec<Value>) -> Value { Value::List(list) }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Ord, PartialOrd, JsonSchema)]
#[schemars(with = "String")]
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

/// The declaration for a resource, typically something like a wordlist or
/// environment variable.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct ResourceDeclaration {
    /// A resource who's default value is specified inline.
    pub inline: Option<String>,
    /// A resource who's default value is meant to be loaded from a file.
    pub path: Option<String>,
    #[serde(rename = "type", default)]
    pub ty: ResourceType,
}

impl ResourceDeclaration {
    pub fn span(&self) -> Span {
        // TODO: Get span from serde_yaml
        Span::default()
    }
}

/// How the resource should be treated inside the Rune.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "kebab-case")]
pub enum ResourceType {
    /// The resource should be treated like as a `&str`.
    String,
    /// The resource should be treated like a `&[u8]`.
    Binary,
}

impl Default for ResourceType {
    fn default() -> Self { ResourceType::String }
}

/// A reference to some [`ResourceDeclaration`]. It typically looks like
/// `$RESOURCE_NAME`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, schemars::JsonSchema)]
pub struct ResourceName(pub String);

impl ResourceName {
    pub fn span(&self) -> Span {
        // TODO: Get span from serde_yaml
        Span::default()
    }
}

impl<S: Into<String>> From<S> for ResourceName {
    fn from(s: S) -> Self { ResourceName(s.into()) }
}

impl FromStr for ResourceName {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("$") {
            return Err("resource names always start with a \"$\"".into());
        }

        if !RESOURCE_NAME_PATTERN.is_match(s) {
            return Err("should be a valid identifier".into());
        }

        Ok(ResourceName(s[1..].to_string()))
    }
}

impl Deref for ResourceName {
    type Target = String;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Serialize for ResourceName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ResourceName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let repr = Cow::<str>::deserialize(deserializer)?;

        if !repr.starts_with("$") {
            return Err(D::Error::custom(
                "resource names always start with a \"$\"",
            ));
        }

        let name = &repr[1..];

        if name.is_empty() {
            Err(D::Error::custom("the resource name is empty"))
        } else if !RESOURCE_NAME_PATTERN.is_match(name) {
            Err(D::Error::custom("should be a valid identifier"))
        } else {
            Ok(ResourceName(name.to_string()))
        }
    }
}

impl Display for ResourceName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "${}", self.0)
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
            (
                "hotg-ai/rune@v1.2#proc_blocks/normalize",
                Path::new(
                    "hotg-ai/rune",
                    "proc_blocks/normalize".to_string(),
                    "v1.2".to_string(),
                ),
            ),
        ];

        for (src, should_be) in inputs {
            let got: Path = src.parse().unwrap();
            assert_eq!(got, should_be, "{}", src);
        }
    }

    #[test]
    fn parse_v1() {
        let src = "version: 1\nimage: asdf\npipeline: {}";

        let got = Document::parse(src).unwrap();

        assert!(matches!(got, Document::V1 { .. }));
    }

    #[test]
    #[should_panic = "expected version to be 1"]
    fn other_versions_are_an_error() {
        let src = "image: asdf\nversion: 2\npipeline:";

        let got = Document::parse(src).unwrap();

        assert!(matches!(got, Document::V1 { .. }));
    }

    #[test]
    fn inline_resource() {
        let src = "inline: some data";
        let should_be = ResourceDeclaration {
            inline: Some(String::from("some data")),
            ..Default::default()
        };

        let got: ResourceDeclaration = serde_yaml::from_str(src).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn resource_from_disk() {
        let src = "path: ./input.txt";
        let should_be = ResourceDeclaration {
            path: Some(String::from("./input.txt")),
            ..Default::default()
        };

        let got: ResourceDeclaration = serde_yaml::from_str(src).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn resource_with_no_default_value() {
        let src = "resource_name: {}";
        let should_be = ResourceDeclaration::default();

        let got: IndexMap<String, ResourceDeclaration> =
            serde_yaml::from_str(src).unwrap();

        let declaration = &got[0];
        assert_eq!(declaration, &should_be);
    }

    #[test]
    fn model_name_from_resource() {
        let src = "$MODEL";
        let should_be = ResourceOrString::Resource("MODEL".into());

        let got: ResourceOrString = serde_yaml::from_str(src).unwrap();

        assert_eq!(got, should_be);

        let round_tripped = serde_yaml::to_string(&got).unwrap();
        assert_eq!(round_tripped, "---\n$MODEL\n");
    }

    #[test]
    #[should_panic = "should be a valid identifier"]
    fn model_name_from_resource_must_not_be_empty() {
        let src = "$";

        let _: ResourceOrString = serde_yaml::from_str(src).unwrap();
    }

    #[test]
    #[should_panic = "should be a valid identifier"]
    fn model_name_from_resource_must_be_valid_identifier() {
        let src = "$";

        let _: ResourceOrString = serde_yaml::from_str(src).unwrap();
    }

    #[test]
    fn model_name_from_path() {
        let src = "./path";
        let should_be = ResourceOrString::String(String::from(src));

        let got: ResourceOrString = serde_yaml::from_str(src).unwrap();

        assert_eq!(got, should_be);

        let round_tripped = serde_yaml::to_string(&got).unwrap();
        assert_eq!(round_tripped, "---\n\"./path\"\n");
    }

    #[test]
    fn proc_block_with_resource_for_arg() {
        let src = r#"
              some-proc-block:
                proc-block: normalize
                outputs:
                - type: u8
                  dimensions: [1]
                args:
                  word-list: $WORD_LIST
            "#;
        let should_be = Stage::ProcBlock {
            proc_block: "normalize".parse().unwrap(),
            inputs: Vec::new(),
            outputs: vec![Type {
                name: String::from("u8"),
                dimensions: vec![1],
            }],
            args: vec![(
                "word-list".to_string(),
                Value::from(ResourceName::from_str("$WORD_LIST").unwrap()),
            )]
            .into_iter()
            .collect(),
        };

        let got: IndexMap<String, Stage> = serde_yaml::from_str(src).unwrap();

        let got = &got["some-proc-block"];
        assert_eq!(got, &should_be);
    }

    #[test]
    fn parse_yaml_pipeline() {
        let src = r#"
version: 1
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
        let should_be = Document::V1(DocumentV1 {
            image: "runicos/base".parse().unwrap(),
            pipeline: map! {
                audio: Stage::Capability {
                    capability: String::from("SOUND"),
                    outputs: vec![ty!(i16[16000])],
                    args: map! { hz: Value::Int(16000) },
                },
                output: Stage::Out {
                    out: String::from("SERIAL"),
                    args: IndexMap::new(),
                    inputs: vec!["label".parse().unwrap()],
                },
                label: Stage::ProcBlock {
                    proc_block: "hotg-ai/rune#proc_blocks/ohv_label".parse().unwrap(),
                    inputs: vec!["model".parse().unwrap()],
                    outputs: vec![Type { name: String::from("utf8"), dimensions: Vec::new() }],
                    args: map! {
                        labels: Value::from(vec![
                            Value::from("silence"),
                            Value::from("unknown"),
                            Value::from("up"),
                            Value::from("down"),
                            Value::from("left"),
                            Value::from("right"),
                        ]),
                    },
                },
                fft: Stage::ProcBlock {
                    proc_block: "hotg-ai/rune#proc_blocks/fft".parse().unwrap(),
                    inputs: vec!["audio".parse().unwrap()],
                    outputs: vec![ty!(i8[1960])],
                    args: IndexMap::new(),
                },
                model: Stage::Model {
                    model: "./model.tflite".into(),
                    inputs: vec!["fft".parse().unwrap()],
                    outputs: vec![ty!(i8[6])],
                },
            },
            resources: map![],
        });

        let got = Document::parse(src).unwrap();

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
                name: String::from("i16"),
                dimensions: vec![16000],
            }],
            args: map! { hz: Value::Int(16000) },
        };

        let got: Stage = serde_yaml::from_str(src).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_values() {
        let inputs = vec![
            ("42", Value::Int(42)),
            ("3.14", Value::Float(3.14)),
            ("\"42\"", Value::String("42".into())),
            (
                "[1, 2.0, \"asdf\"]",
                Value::List(vec![
                    Value::Int(1),
                    Value::Float(2.0),
                    Value::String("asdf".into()),
                ]),
            ),
        ];

        for (src, should_be) in inputs {
            let got: Value = serde_yaml::from_str(src).unwrap();
            assert_eq!(got, should_be);
        }
    }
}

/// The image a Rune is based on.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct Image(pub Path);

impl FromStr for Image {
    type Err = PathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Path::from_str(s).map(Image)
    }
}

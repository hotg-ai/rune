//! Definitions for the Runefile's YAML format.

use std::{
    borrow::Cow,
    fmt::{self, Formatter, Display},
    ops::Deref,
    str::FromStr,
};
use schemars::{
    JsonSchema,
    gen::SchemaGenerator,
    schema::{
        InstanceType, Schema, SchemaObject, Metadata, SubschemaValidation,
    },
};
use indexmap::IndexMap;
use regex::Regex;
use once_cell::sync::Lazy;
use serde::{
    de::{Deserialize, Deserializer, Error as _},
    ser::{Serialize, Serializer},
};
use codespan::Span;

static RESOURCE_NAME_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\$[_a-zA-Z][_a-zA-Z0-9]*$").unwrap());

/// The top level Runefile type.
#[derive(Debug, Clone, PartialEq, JsonSchema)]
#[schemars(untagged)]
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

macro_rules! impl_json_schema_via_regex {
    ($ty:ty, $pattern:expr, $docs:literal) => {
        impl JsonSchema for $ty {
            fn schema_name() -> String { String::from(stringify!($ty)) }

            fn json_schema(_: &mut SchemaGenerator) -> Schema {
                let mut schema = SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    format: Some(String::from("string")),
                    metadata: Some(Box::new(Metadata {
                        description: Some(String::from($docs)),
                        ..Default::default()
                    })),
                    ..Default::default()
                };

                schema.string().pattern = Some($pattern.to_string());

                schema.into()
            }
        }
    };
}

/// Version 1 of the `Runefile.yml` format.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct DocumentV1 {
    /// The version number. Must always be `"1"`.
    #[schemars(required, range(min = 1, max = 1))]
    pub version: usize,
    /// The base image that defines the interface between a Rune and its
    /// runtime.
    ///
    /// This should always be `"runicos/base"`.
    pub image: Image,
    /// The various stages in the Runefile's pipeline.
    pub pipeline: IndexMap<String, Stage>,
    /// Any resources that can be accessed by pipeline stages.
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Path {
    pub base: String,
    pub sub_path: Option<String>,
    pub version: Option<String>,
}

impl_json_schema_via_regex!(
    Path,
    PATH_PATTERN,
    r#"
A specification for finding a dependency.

The full syntax is `base@version#sub_path` where

- `base` is a URL or the name of a repository on GitHub (e.g. `hotg-ai/rune`
  or `https://github.com/hotg-ai/rune`)
- `version` is an optional field specifying the version (e.g. as a git tag)
- `sub_path` is an optional field which is useful when pointing to
  repositories with multiple relevant items because it lets you specify
  which directory the specified item is in.
"#
);

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
        if let Some(version) = version {
            write!(f, "@{}", version)?;
        }
        if let Some(sub) = sub_path {
            write!(f, "#{}", sub)?;
        }

        Ok(())
    }
}

static PATH_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)
        (?P<base>[\w\d:/_.-]+)
        (?:@(?P<version>[\w\d./-]+))?
        (?:\#(?P<sub_path>[\w\d._/-]+))?
        ",
    )
    .unwrap()
});

impl FromStr for Path {
    type Err = PathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = PATH_PATTERN.captures(s).ok_or(PathParseError)?;

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

/// A ML model which will be executed by the runtime.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct ModelStage {
    /// The model to use, or a resource which specifies the model to use.
    #[schemars(required)]
    pub model: ResourceOrString,
    /// Tensors to use as input to this model.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<Input>,
    /// The tensors that this model outputs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<Type>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub args: IndexMap<String, Argument>,
}

/// A stage which executes a procedural block.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct ProcBlockStage {
    /// A [`Path`] that Rune can use to locate the proc block.
    #[serde(rename = "proc-block")]
    #[schemars(required)]
    pub proc_block: Path,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<Input>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<Type>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub args: IndexMap<String, Argument>,
}

/// A stage which reads inputs from the runtime.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct CapabilityStage {
    /// What type of capability to use ("IMAGE", "SOUND", etc.).
    #[schemars(required)]
    pub capability: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<Type>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub args: IndexMap<String, Argument>,
}

/// A stage which passes outputs back to the runtime.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct OutStage {
    /// The type of output (e.g. "SERIAL").
    #[schemars(required)]
    pub out: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<Input>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub args: IndexMap<String, Argument>,
}

/// A stage in the Rune's pipeline.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema,
)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum Stage {
    Model(ModelStage),
    ProcBlock(ProcBlockStage),
    Capability(CapabilityStage),
    Out(OutStage),
}

impl Stage {
    pub fn inputs(&self) -> &[Input] {
        match self {
            Stage::Model(ModelStage { inputs, .. })
            | Stage::ProcBlock(ProcBlockStage { inputs, .. })
            | Stage::Out(OutStage { inputs, .. }) => inputs,
            Stage::Capability(_) => &[],
        }
    }

    pub fn inputs_mut(&mut self) -> Option<&mut Vec<Input>> {
        match self {
            Stage::Model(ModelStage { inputs, .. })
            | Stage::ProcBlock(ProcBlockStage { inputs, .. })
            | Stage::Out(OutStage { inputs, .. }) => Some(inputs),
            Stage::Capability(_) => None,
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
            Stage::Model(ModelStage { outputs, .. })
            | Stage::ProcBlock(ProcBlockStage { outputs, .. })
            | Stage::Capability(CapabilityStage { outputs, .. }) => outputs,
            Stage::Out(OutStage { .. }) => &[],
        }
    }

    pub fn span(&self) -> Span {
        // TODO: Get span from serde_yaml
        Span::default()
    }

    pub fn args(&self) -> &IndexMap<String, Argument> {
        match self {
            Stage::Model(m) => &m.args,
            Stage::ProcBlock(p) => &p.args,
            Stage::Capability(c) => &c.args,
            Stage::Out(out) => &out.args,
        }
    }
}

/// Something that could be either a reference to a resource (`$resource`)
/// or a plain string (`./path`).
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceOrString {
    Resource(ResourceName),
    String(String),
}

impl JsonSchema for ResourceOrString {
    fn schema_name() -> std::string::String { "ResourceOrString".to_owned() }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let resource_name = gen.subschema_for::<ResourceName>();
        let string = gen.subschema_for::<String>();

        let description = "Something that could be either a reference to a resource (`$resource`) or a plain string (`./path`)." ;

        Schema::Object(SchemaObject {
            metadata: Some(Box::new(Metadata {
                description: Some(description.to_owned()),
                ..Default::default()
            })),
            subschemas: Some(Box::new(SubschemaValidation {
                any_of: Some(vec![resource_name, string]),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
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
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResourceOrString;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a number, string, or \"$RESOURCE_NAME\"")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ResourceOrString::String(v.to_string()))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ResourceOrString::String(v.to_string()))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ResourceOrString::String(v.to_string()))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let v = v.trim();

                if !v.starts_with('$') {
                    return Ok(ResourceOrString::String(v.to_string()));
                }

                match ResourceName::from_str(v) {
                    Ok(name) => Ok(ResourceOrString::Resource(name)),
                    Err(e) => Err(E::custom(e)),
                }
            }

            fn visit_seq<A>(self, _: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                Err(A::Error::custom("lists aren't supported"))
            }
        }

        deserializer.deserialize_any(Visitor)
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

/// A newtype around [`ResourceOrString`] which is used in each stage's `args`
/// dictionary.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Argument(pub ResourceOrString);

impl JsonSchema for Argument {
    fn schema_name() -> std::string::String { "Argument".to_owned() }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let number = gen.subschema_for::<serde_json::Number>();

        let mut schema = ResourceOrString::json_schema(gen).into_object();
        schema.subschemas().any_of.as_mut().unwrap().push(number);

        schema.into()
    }
}

impl<T: Into<ResourceOrString>> From<T> for Argument {
    fn from(value: T) -> Self { Argument(value.into()) }
}

impl Deref for Argument {
    type Target = ResourceOrString;

    fn deref(&self) -> &Self::Target { &self.0 }
}

/// The element type and dimensions for a particular tensor.
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

/// The name of a tensor.
///
/// Typically something like "stage", or "stage.2" if the stage has multiple
/// outputs.
#[derive(Debug, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct Input {
    pub name: String,
    pub index: Option<usize>,
}

impl_json_schema_via_regex!(
    Input,
    INPUT_PATTERN,
    r#"
The name of a tensor.

Typically something like "stage", or "stage.2" if the stage has multiple outputs.
"#
);

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

static INPUT_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?P<name>[a-zA-Z_][\w-]*)(?:\.(?P<index>\d+))?$").unwrap()
});

impl FromStr for Input {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = INPUT_PATTERN
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceName(pub String);

impl_json_schema_via_regex!(
    ResourceName,
    RESOURCE_NAME_PATTERN,
    r#"
A reference to some [`ResourceDeclaration`]. It typically looks like
`$RESOURCE_NAME`.
"#
);

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
        if !s.starts_with('$') {
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

        if !repr.starts_with('$') {
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

/// The image a Rune is based on.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[schemars(transparent)]
pub struct Image(pub Path);

impl FromStr for Image {
    type Err = PathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Path::from_str(s).map(Image)
    }
}

#[cfg(test)]
mod tests {
    use jsonschema::JSONSchema;

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
            let round_tripped = got.to_string();
            assert_eq!(round_tripped, src);
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
        let should_be = Stage::ProcBlock(ProcBlockStage {
            proc_block: "normalize".parse().unwrap(),
            inputs: Vec::new(),
            outputs: vec![Type {
                name: String::from("u8"),
                dimensions: vec![1],
            }],
            args: vec![(
                "word-list".to_string(),
                ResourceName::from_str("$WORD_LIST").unwrap().into(),
            )]
            .into_iter()
            .collect(),
        });

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
      labels: |
        silence
        unknown
        up
        down
        left
        right

  output:
    out: SERIAL
    inputs:
    - label
        "#;
        let should_be = Document::V1(DocumentV1 {
            version: 1,
            image: "runicos/base".parse().unwrap(),
            pipeline: map! {
                audio: Stage::Capability(CapabilityStage {
                    capability: String::from("SOUND"),
                    outputs: vec![ty!(i16[16000])],
                    args: map! { hz: "16000".into() },
                }),
                fft: Stage::ProcBlock(ProcBlockStage {
                    proc_block: "hotg-ai/rune#proc_blocks/fft".parse().unwrap(),
                    inputs: vec!["audio".parse().unwrap()],
                    outputs: vec![ty!(i8[1960])],
                    args: IndexMap::new(),
                }),
                model: Stage::Model(ModelStage {
                    model: "./model.tflite".into(),
                    inputs: vec!["fft".parse().unwrap()],
                    outputs: vec![ty!(i8[6])],
                    args: IndexMap::new(),
                }),
                label: Stage::ProcBlock(ProcBlockStage {
                    proc_block: "hotg-ai/rune#proc_blocks/ohv_label".parse().unwrap(),
                    inputs: vec!["model".parse().unwrap()],
                    outputs: vec![Type { name: String::from("utf8"), dimensions: Vec::new() }],
                    args: map! {
                        labels: "silence\nunknown\nup\ndown\nleft\nright".into()
                    },
                }),
                output: Stage::Out(OutStage {
                    out: String::from("SERIAL"),
                    args: IndexMap::new(),
                    inputs: vec!["label".parse().unwrap()],
                }),
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
        let should_be = Stage::Capability(CapabilityStage {
            capability: String::from("SOUND"),
            outputs: vec![Type {
                name: String::from("i16"),
                dimensions: vec![16000],
            }],
            args: map! { hz: "16000".into() },
        });

        let got: Stage = serde_yaml::from_str(src).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn schema_is_in_sync_with_version_on_disk() {
        let existing_schema = include_str!("../../runefile-schema.json");
        let should_be: serde_json::Value =
            serde_json::from_str(existing_schema).unwrap();

        let schema = schemars::schema_for!(Document);

        let schema = serde_json::to_value(&schema).unwrap();
        assert_eq!(
            should_be,
            schema,
            "The schema is out of sync. You probably need to run \"cargo xtask update-schema\"",
        );
    }

    #[track_caller]
    fn handle_errors<'a>(
        errors: impl Iterator<Item = jsonschema::ValidationError<'a>>,
    ) -> ! {
        for err in errors {
            println!("{}", err);
        }

        panic!("Validation failed");
    }

    #[test]
    fn argument_schema_is_valid() {
        let schema = schemars::schema_for!(Argument);
        let schema_json = serde_json::to_value(&schema).unwrap();
        let compiled_schema =
            JSONSchema::options().compile(&schema_json).unwrap();

        let string = serde_json::Value::String("".to_string());
        compiled_schema
            .validate(&string)
            .unwrap_or_else(|e| handle_errors(e));

        let resource = serde_json::Value::String("$resource".to_string());
        compiled_schema
            .validate(&resource)
            .unwrap_or_else(|e| handle_errors(e));

        let number = serde_json::Value::Number(10.into());
        compiled_schema
            .validate(&number)
            .unwrap_or_else(|e| handle_errors(e));
    }
}

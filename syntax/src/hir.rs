//! The *High-level Internal Representation*.

use std::{
    collections::{HashMap, HashSet},
    convert::{TryFrom},
    hash::Hash,
    io::{Error, ErrorKind, Write},
    ops::Index,
    path::PathBuf,
};
use codespan::Span;
use indexmap::IndexMap;
use crate::{ast::Path, yaml::Value};

#[derive(
    Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct Rune {
    pub base_image: Option<Path>,
    pub stages: IndexMap<HirId, Node>,
    pub slots: IndexMap<HirId, Slot>,
    pub types: IndexMap<HirId, Type>,
    pub names: NameTable,
    pub spans: IndexMap<HirId, Span>,
}

impl Rune {
    pub fn stages(&self) -> impl Iterator<Item = (HirId, &Stage)> + '_ {
        self.stages.iter().map(|(id, node)| (*id, &node.stage))
    }

    pub fn proc_blocks(
        &self,
    ) -> impl Iterator<Item = (HirId, &ProcBlock)> + '_ {
        self.stages().filter_map(|(h, stage)| match stage {
            Stage::ProcBlock(pb) => Some((h, pb)),
            _ => None,
        })
    }

    pub fn models(&self) -> impl Iterator<Item = (HirId, &Model)> + '_ {
        self.stages().filter_map(|(h, stage)| match stage {
            Stage::Model(m) => Some((h, m)),
            _ => None,
        })
    }

    pub fn sinks(&self) -> impl Iterator<Item = (HirId, &Sink)> + '_ {
        self.stages().filter_map(|(h, stage)| match stage {
            Stage::Sink(s) => Some((h, s)),
            _ => None,
        })
    }

    pub fn sources(&self) -> impl Iterator<Item = (HirId, &Source)> + '_ {
        self.stages().filter_map(|(h, stage)| match stage {
            Stage::Source(s) => Some((h, s)),
            _ => None,
        })
    }

    pub fn has_connection(&self, from: HirId, to: HirId) -> bool {
        !self.connecting_slots(from, to).is_empty()
    }

    pub fn connecting_slots(&self, from: HirId, to: HirId) -> HashSet<HirId> {
        let from_node = match self.stages.get(&from) {
            Some(n) => n,
            None => return HashSet::new(),
        };
        let to_node = match self.stages.get(&to) {
            Some(n) => n,
            None => return HashSet::new(),
        };

        let previous_outputs: HashSet<_> =
            from_node.output_slots.iter().collect();
        let next_inputs: HashSet<_> = to_node.input_slots.iter().collect();

        previous_outputs
            .intersection(&next_inputs)
            .copied()
            .copied()
            .collect()
    }

    /// Get a topological sorting of the pipeline graph.
    pub fn sorted_pipeline(&self) -> impl Iterator<Item = (HirId, &Node)> + '_ {
        // https://www.geeksforgeeks.org/topological-sorting/

        let mut visited = HashSet::with_capacity(self.stages.len());
        let mut stack = Vec::with_capacity(self.stages.len());

        for key in self.stages.keys() {
            if !visited.contains(key) {
                topo_sort(*key, self, &mut visited, &mut stack);
            }
        }

        stack.into_iter().map(move |id| (id, &self.stages[&id]))
    }
}

fn topo_sort(
    id: HirId,
    rune: &Rune,
    visited: &mut HashSet<HirId>,
    stack: &mut Vec<HirId>,
) {
    visited.insert(id);

    let node = &rune.stages[&id];

    for incoming in &node.input_slots {
        let slot = &rune.slots[incoming];
        let input = slot.input_node;

        if !visited.contains(&input) {
            topo_sort(input, rune, visited, stack);
        }
    }

    stack.push(id);
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct HirId(u32);

impl HirId {
    pub const ERROR: HirId = HirId(0);
    pub const UNKNOWN: HirId = HirId(1);

    /// The first non-builtin [`HirId`] that can be allocated to a HIR object.
    pub(crate) const fn first_user_defined() -> HirId { HirId(2) }

    pub fn is_error(self) -> bool { self == HirId::ERROR }

    pub fn is_unknown(self) -> bool { self == HirId::UNKNOWN }

    pub(crate) fn next(self) -> Self { HirId(self.0 + 1) }
}

impl Default for HirId {
    fn default() -> Self { HirId::first_user_defined() }
}

/// A table mapping names to [`HirId`]s.
#[derive(
    Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct NameTable {
    name_to_id: HashMap<String, HirId>,
    id_to_name: HashMap<HirId, String>,
}

impl NameTable {
    pub fn register(&mut self, name: &str, id: HirId) -> Result<(), HirId> {
        if let Some(existing_item_id) = self.get_id(name) {
            return Err(existing_item_id);
        }

        self.name_to_id.insert(name.to_string(), id);
        self.id_to_name.insert(id, name.to_string());

        Ok(())
    }

    pub fn get_name(&self, id: HirId) -> Option<&str> {
        self.id_to_name.get(&id).map(|s| s.as_str())
    }

    pub fn get_id(&self, name: &str) -> Option<HirId> {
        self.name_to_id.get(name).copied()
    }
}

impl Index<HirId> for NameTable {
    type Output = str;

    #[track_caller]
    fn index(&self, index: HirId) -> &Self::Output {
        self.get_name(index).unwrap()
    }
}

impl<S: AsRef<str>> Index<S> for NameTable {
    type Output = HirId;

    #[track_caller]
    fn index(&self, index: S) -> &Self::Output {
        self.name_to_id.get(index.as_ref()).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Stage {
    Source(Source),
    Sink(Sink),
    Model(Model),
    ProcBlock(ProcBlock),
}

impl From<Sink> for Stage {
    fn from(s: Sink) -> Self { Stage::Sink(s) }
}

impl TryFrom<Stage> for Sink {
    type Error = ();

    fn try_from(stage: Stage) -> Result<Self, Self::Error> {
        match stage {
            Stage::Sink(s) => Ok(s),
            _ => Err(()),
        }
    }
}

impl From<Source> for Stage {
    fn from(s: Source) -> Self { Stage::Source(s) }
}

impl TryFrom<Stage> for Source {
    type Error = ();

    fn try_from(stage: Stage) -> Result<Self, Self::Error> {
        match stage {
            Stage::Source(s) => Ok(s),
            _ => Err(()),
        }
    }
}

impl From<Model> for Stage {
    fn from(m: Model) -> Self { Stage::Model(m) }
}

impl TryFrom<Stage> for Model {
    type Error = ();

    fn try_from(stage: Stage) -> Result<Self, Self::Error> {
        match stage {
            Stage::Model(m) => Ok(m),
            _ => Err(()),
        }
    }
}

impl From<ProcBlock> for Stage {
    fn from(p: ProcBlock) -> Self { Stage::ProcBlock(p) }
}

impl TryFrom<Stage> for ProcBlock {
    type Error = ();

    fn try_from(stage: Stage) -> Result<Self, Self::Error> {
        match stage {
            Stage::ProcBlock(pb) => Ok(pb),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Sink {
    pub kind: SinkKind,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum SinkKind {
    Serial,
    Other(String),
}

impl<'a> From<&'a str> for SinkKind {
    fn from(s: &'a str) -> SinkKind {
        match s {
            "serial" | "SERIAL" => SinkKind::Serial,
            _ => SinkKind::Other(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Model {
    pub model_file: PathBuf,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Type {
    Primitive(Primitive),
    /// The concrete type isn't yet known.
    Unknown,
    /// A multidimensional array of data.
    Buffer {
        underlying_type: HirId,
        dimensions: Vec<usize>,
    },
    /// This can be *any* type.
    Any,
}

impl Type {
    pub fn underlying_primitive(
        &self,
        types: &HashMap<HirId, Type>,
    ) -> Option<Primitive> {
        match self {
            Type::Primitive(p) => Some(*p),
            Type::Buffer {
                underlying_type, ..
            } => {
                let underlying_type = types.get(underlying_type)?;
                underlying_type.underlying_primitive(types)
            },
            Type::Unknown | Type::Any => None,
        }
    }

    pub fn rust_type_name(
        &self,
        types: &HashMap<HirId, Type>,
    ) -> Result<String, Error> {
        let mut buffer = Vec::new();
        self.write_rust_type_name(&mut buffer, types)?;
        let name = String::from_utf8(buffer)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        Ok(name)
    }

    fn write_rust_type_name<W: Write>(
        &self,
        w: &mut W,
        types: &HashMap<HirId, Type>,
    ) -> Result<(), Error> {
        match self {
            Type::Primitive(p) => write!(w, "{}", p.rust_name()),
            Type::Buffer {
                underlying_type,
                dimensions,
            } => write_rust_array_type_name(
                w,
                *underlying_type,
                dimensions,
                types,
            ),
            Type::Any | Type::Unknown => Err(Error::new(
                ErrorKind::Other,
                "The concrete type isn't known",
            )),
        }
    }
}

fn write_rust_array_type_name<W: Write>(
    w: &mut W,
    underlying_type: HirId,
    dimensions: &[usize],
    types: &HashMap<HirId, Type>,
) -> Result<(), Error> {
    match dimensions.split_first() {
        Some((dim, rest)) => {
            write!(w, "[")?;
            write_rust_array_type_name(w, underlying_type, rest, types)?;

            write!(w, "; {}]", dim)?;
            Ok(())
        },
        None => {
            let ty = types
                .get(&underlying_type)
                .ok_or_else(|| Error::new(ErrorKind::Other, "Unknown type"))?;
            ty.write_rust_type_name(w, types)?;
            Ok(())
        },
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Primitive {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    String,
}

impl Primitive {
    pub fn rust_name(self) -> &'static str {
        match self {
            Primitive::U8 => "u8",
            Primitive::I8 => "i8",
            Primitive::U16 => "u16",
            Primitive::I16 => "i16",
            Primitive::U32 => "u32",
            Primitive::I32 => "i32",
            Primitive::U64 => "u64",
            Primitive::I64 => "i64",
            Primitive::F32 => "f32",
            Primitive::F64 => "f64",
            Primitive::String => "&'static str",
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Source {
    pub kind: SourceKind,
    pub parameters: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum SourceKind {
    Random,
    Accelerometer,
    Sound,
    Image,
    Raw,
    Other(String),
}

impl<'a> From<&'a str> for SourceKind {
    fn from(s: &'a str) -> SourceKind {
        match s {
            "rand" | "RAND" => SourceKind::Random,
            "accel" | "ACCEL" => SourceKind::Accelerometer,
            "sound" | "SOUND" => SourceKind::Sound,
            "image" | "IMAGE" => SourceKind::Image,
            "raw" | "RAW" => SourceKind::Raw,
            _ => SourceKind::Other(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProcBlock {
    pub path: Path,
    pub parameters: HashMap<String, Value>,
}

impl ProcBlock {
    pub fn name(&self) -> &str {
        let full_name = self.path.sub_path.as_ref().unwrap_or(&self.path.base);

        let start_of_name = full_name.rfind("/").map(|ix| ix + 1).unwrap_or(0);

        &full_name[start_of_name..]
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pipeline {
    /// The edges associated with this pipeline.
    pub edges: HashSet<HirId>,
}

#[derive(
    Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Edge {
    pub type_id: HirId,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PipelineGraph {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Node {
    pub stage: Stage,
    /// The [`Slot`]s that this [`Node`] receives data from.
    pub input_slots: Vec<HirId>,
    /// The [`Slot`]s that this [`Node`] sends data to.
    pub output_slots: Vec<HirId>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Slot {
    pub element_type: HirId,
    pub input_node: HirId,
    pub output_node: HirId,
}

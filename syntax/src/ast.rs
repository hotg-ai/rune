use codespan::Span;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Runefile {
    pub instructions: Vec<Instruction>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    From(FromInstruction),
    Model(ModelInstruction),
    Capability(CapabilityInstruction),
    Run(RunInstruction),
    ProcBlock(ProcBlockInstruction),
    Out(OutInstruction),
}

impl From<FromInstruction> for Instruction {
    fn from(other: FromInstruction) -> Self { Instruction::From(other) }
}

impl From<ModelInstruction> for Instruction {
    fn from(other: ModelInstruction) -> Self { Instruction::Model(other) }
}

impl From<CapabilityInstruction> for Instruction {
    fn from(other: CapabilityInstruction) -> Self {
        Instruction::Capability(other)
    }
}

impl From<RunInstruction> for Instruction {
    fn from(other: RunInstruction) -> Self { Instruction::Run(other) }
}

impl From<ProcBlockInstruction> for Instruction {
    fn from(other: ProcBlockInstruction) -> Self {
        Instruction::ProcBlock(other)
    }
}

impl From<OutInstruction> for Instruction {
    fn from(other: OutInstruction) -> Self { Instruction::Out(other) }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FromInstruction {
    pub image: Ident,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModelInstruction {
    pub model_name: Ident,
    pub model_file: String,
    pub model_parameters: HashMap<String, String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CapabilityInstruction {
    pub capability_name: Ident,
    pub capability_description: String,
    pub capability_parameters: HashMap<String, String>,
    pub dependencies: HashMap<String, String>,
    pub input_type: Type,
    pub output_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Inferred,
    Named(Ident),
}

#[derive(Debug, PartialEq, Clone)]
pub struct RunInstruction {
    pub steps: Vec<String>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProcBlockInstruction {
    pub path: String,
    pub name: Ident,
    pub params: HashMap<String, String>,
    pub dependencies: HashMap<String, String>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OutInstruction {
    pub out_type: Ident,
    pub span: Span,
}

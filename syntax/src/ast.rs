//! The *Abstract Syntax Tree* for a Runefile.

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

impl Instruction {
    pub fn span(&self) -> Span {
        match self {
            Instruction::From(f) => f.span,
            Instruction::Model(m) => m.span,
            Instruction::Capability(c) => c.span,
            Instruction::Run(r) => r.span,
            Instruction::ProcBlock(p) => p.span,
            Instruction::Out(o) => o.span,
        }
    }
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
    pub name: Ident,
    pub file: String,
    pub parameters: HashMap<String, String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub value: String,
    pub span: Span,
}

impl Ident {
    pub fn new(value: impl Into<String>, span: Span) -> Self {
        Ident {
            value: value.into(),
            span,
        }
    }

    /// Create an [`Ident`] with a placeholder span.
    #[cfg(test)]
    pub(crate) fn dangling(value: impl Into<String>) -> Self {
        Ident::new(value, Span::new(0, 0))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CapabilityInstruction {
    pub name: Ident,
    pub description: String,
    pub parameters: HashMap<String, String>,
    pub input_type: Type,
    pub output_type: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
}

#[cfg(test)]
impl Type {
    pub(crate) fn named_dangling(name: impl Into<String>) -> Self {
        Type {
            kind: TypeKind::Named(Ident::dangling(name)),
            span: Span::new(0, 0),
        }
    }

    pub(crate) fn inferred_dangling() -> Self {
        Type {
            kind: TypeKind::Inferred,
            span: Span::new(0, 0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Inferred,
    Named(Ident),
}

#[derive(Debug, PartialEq, Clone)]
pub struct RunInstruction {
    pub steps: Vec<Ident>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProcBlockInstruction {
    pub path: String,
    pub name: Ident,
    pub params: HashMap<String, String>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OutInstruction {
    pub out_type: Ident,
    pub span: Span,
}

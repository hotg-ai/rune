use proc_macro2::Ident;
use syn::{Generics, Path, Type};
use crate::descriptor::ProcBlockDescriptor;

#[derive(Debug)]
pub(crate) struct DeriveOutput {
    pub trait_impl: ProcBlockImpl,
    pub custom_section: CustomSection,
    pub setters: Setters,
    pub assertions: Assertions,
}

#[derive(Debug)]
pub(crate) struct ProcBlockImpl {
    pub type_name: Ident,
    pub exports: Path,
    pub generics: Generics,
    pub descriptor: ProcBlockDescriptor<'static>,
}

#[derive(Debug)]
pub(crate) struct CustomSection {
    pub type_name: Ident,
    pub payload: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Setters {
    pub type_name: Ident,
    pub generics: Generics,
    pub setters: Vec<Setter>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Setter {
    pub property: Ident,
    pub property_type: syn::Type,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Assertions {
    pub set: SetterAssertions,
    pub transform: TransformAssertions,
}

#[derive(Debug, PartialEq)]
pub(crate) struct SetterAssertions(pub Vec<SetterAssertion>);

#[derive(Debug, PartialEq)]
pub(crate) struct SetterAssertion {
    pub proc_block_type: Ident,
    pub property: Ident,
    pub setter_argument: Type,
}

#[derive(Debug, PartialEq)]
pub(crate) struct TransformAssertions {
    pub proc_block_type: Ident,
    pub exports: Path,
    pub assertions: Vec<TransformAssertion>,
}

/// An assertion that our type implements `Transform<$input, Output=$output>`.
#[derive(Debug, PartialEq)]
pub(crate) struct TransformAssertion {
    pub inputs: Vec<Type>,
    pub outputs: Vec<Type>,
}

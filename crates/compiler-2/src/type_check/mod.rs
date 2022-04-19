//! The Rune type checker.
mod query;
mod types;

pub use self::{
    query::{TypeCheck, TypeCheckGroup},
    types::*,
};

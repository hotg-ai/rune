//! Parsing and analysis of Runefiles.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod analysis;
pub mod ast;
mod diagnostics;
pub mod hir;
pub mod parse;
mod utils;
pub mod yaml;

pub use crate::{
    analysis::analyse as analyse_yaml_runefile, diagnostics::Diagnostics,
    parse::parse,
};

pub fn analyse(runefile: &ast::Runefile, diags: &mut Diagnostics) -> hir::Rune {
    let document = yaml::document_from_runefile(runefile, diags);
    analyse_yaml_runefile(&document, diags)
}

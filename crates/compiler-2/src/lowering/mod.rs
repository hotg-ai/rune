//! Convert a Runefile's AST into a high-level intermediate representation that
//! is more amenable to analysis.
//!
//! This high-level intermediate representation is stored as inputs on the
//! [`HirDB`] query group.
//!
//! When loading a Rune from a `Runefile.yml`, you will typically use
//! [`populate_from_document()`] to set the inputs.
//!
//! # Examples
//!
//! ```
//! use hotg_rune_compiler_2::{
//!     lowering::{self, HirDB, HirDBStorage},
//!     parse::parse_runefile,
//! };
//!
//! #[derive(Default)]
//! #[salsa::database(HirDBStorage)]
//! struct Database {
//!     storage: salsa::Storage<Self>,
//! }
//!
//! impl salsa::Database for Database {}
//!
//! let runefile = r#"
//!   version: 1
//!   image: "runicos/base"
//!   pipeline: {}
//!   resources: {}
//! "#;
//! let doc = parse_runefile(runefile).unwrap();
//!
//! let mut db = Database::default();
//! let diags = lowering::populate_from_document(&mut db, doc);
//!
//! assert!(
//!     diags.is_empty(),
//!     "Lowering should have completed without any issues"
//! );
//! ```

pub(crate) mod diagnostics;
mod query;
mod types;

pub use self::{
    diagnostics::*,
    query::{populate_from_document, HirDB, HirDBStorage},
    types::*,
};

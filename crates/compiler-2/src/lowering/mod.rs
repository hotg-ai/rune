//! Convert a Runefile's AST into a high-level intermediate representation that
//! is more amenable to analysis.
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
//! lowering::populate_from_document(&mut db, doc);
//!
//! let diags = db.lowering_diagnostics();
//!
//! assert!(
//!     diags.is_empty(),
//!     "Lowering should have completed without any issues"
//! );
//! ```

mod query;
mod types;

pub use self::{
    query::{populate_from_document, HirDB, HirDBStorage},
    types::*,
};

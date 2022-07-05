use std::{
    collections::HashMap,
    ffi::CString,
    os::raw::{c_char, c_int},
    ptr::{self},
};

use hotg_rune_runtime::NodeMetadata;

/// Metadata for a set of nodes in the ML pipeline.
pub struct Metadata(Vec<Node>);

impl std::ops::Deref for Metadata {
    type Target = [Node];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&'_ HashMap<u32, NodeMetadata>> for Metadata {
    fn from(node_metadata: &'_ HashMap<u32, NodeMetadata>) -> Metadata {
        let mut meta = Vec::new();

        for (&id, m) in node_metadata {
            let NodeMetadata {
                kind, arguments, ..
            } = m;
            let kind = CString::new(kind.as_str()).unwrap();
            let mut args = Vec::new();

            for (key, value) in arguments {
                let key = CString::new(key.as_str()).unwrap();
                let value = CString::new(value.as_str()).unwrap();
                args.push((key, value));
            }

            meta.push(Node {
                id,
                kind,
                arguments: args,
            });
        }

        Metadata(meta)
    }
}

/// Free a `Metadata` when you are done with it.
#[no_mangle]
pub unsafe extern "C" fn rune_metadata_free(meta: *mut Metadata) {
    if meta.is_null() {
        return;
    }

    let _ = Box::from_raw(meta);
}

/// Get metadata for a specific node, returning `null` if the node doesn't
/// exist.
///
/// # Safety
///
/// The returned pointer can't outlive the `Metadata` it came from.
#[no_mangle]
pub unsafe extern "C" fn rune_metadata_get_node(
    meta: *const Metadata,
    index: c_int,
) -> *const Node {
    if meta.is_null() {
        return ptr::null();
    }

    match (&*meta).get(index as usize) {
        Some(n) => n,
        None => ptr::null(),
    }
}

/// How many nodes does this set of `Metadata` contain?
#[no_mangle]
pub unsafe extern "C" fn rune_metadata_node_count(
    meta: *const Metadata,
) -> c_int {
    if meta.is_null() {
        return 0;
    }

    (&*meta).len() as c_int
}

/// Metadata for a single node.
pub struct Node {
    id: u32,
    kind: CString,
    arguments: Vec<(CString, CString)>,
}

/// Get the ID for this particular node.
#[no_mangle]
pub unsafe extern "C" fn rune_node_id(node: *const Node) -> u32 {
    if node.is_null() {
        return u32::MAX;
    }

    (&*node).id
}

/// Which kind of node is this?
///
/// Some examples are `"RAW"` or `"IMAGE"`.
///
/// # Safety
///
/// The returned pointer can't outlive the `Metadata` it came from.
#[no_mangle]
pub unsafe extern "C" fn rune_node_kind(node: *const Node) -> *const c_char {
    if node.is_null() {
        return ptr::null();
    }

    (&*node).kind.as_ptr()
}

/// How many arguments have been passed to this node?
#[no_mangle]
pub unsafe extern "C" fn rune_node_argument_count(node: *const Node) -> c_int {
    if node.is_null() {
        return 0;
    }

    (&*node).arguments.len() as c_int
}

/// Get the name for a particular argument, or `null` if that argument doesn't
/// exist.
#[no_mangle]
pub unsafe extern "C" fn rune_node_get_argument_name(
    node: *const Node,
    index: c_int,
) -> *const c_char {
    if node.is_null() {
        return ptr::null();
    }

    match (&*node).arguments.get(index as usize) {
        Some((key, _)) => key.as_ptr(),
        None => ptr::null(),
    }
}

/// Get the value for a particular argument, or `null` if that argument doesn't
/// exist.
#[no_mangle]
pub unsafe extern "C" fn rune_node_get_argument_value(
    node: *const Node,
    index: c_int,
) -> *const c_char {
    if node.is_null() || index < 0 {
        return ptr::null();
    }

    match (&*node).arguments.get(index as usize) {
        Some((_, value)) => value.as_ptr(),
        None => ptr::null(),
    }
}

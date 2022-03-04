use std::{
    collections::HashMap,
    ffi::CString,
    os::raw::{c_char, c_int},
    ptr::{self, NonNull},
};

use hotg_rune_runtime::NodeMetadata as RustNodeMetadata;

/// Metadata for a set of nodes in the ML pipeline.
pub struct Metadata(Vec<NodeMetadata>);

impl std::ops::Deref for Metadata {
    type Target = [NodeMetadata];

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl From<&'_ HashMap<u32, RustNodeMetadata>> for Metadata {
    fn from(node_metadata: &'_ HashMap<u32, RustNodeMetadata>) -> Metadata {
        let mut meta = Vec::new();

        for (&id, m) in node_metadata {
            let RustNodeMetadata {
                kind, arguments, ..
            } = m;
            let kind = CString::new(kind.as_str()).unwrap();
            let mut args = Vec::new();

            for (key, value) in arguments {
                let key = CString::new(key.as_str()).unwrap();
                let value = CString::new(value.as_str()).unwrap();
                args.push((key, value));
            }

            meta.push(NodeMetadata {
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
    meta: Option<NonNull<Metadata>>,
    index: c_int,
) -> Option<NonNull<NodeMetadata>> {
    match meta?.as_ref().get(index as usize) {
        Some(n) => Some(n.into()),
        None => None,
    }
}

/// How many nodes does this set of `Metadata` contain?
#[no_mangle]
pub unsafe extern "C" fn rune_metadata_node_count(
    meta: Option<NonNull<Metadata>>,
) -> c_int {
    match meta {
        Some(m) => m.as_ref().len() as c_int,
        None => 0,
    }
}

/// Metadata for a single node.
pub struct NodeMetadata {
    id: u32,
    kind: CString,
    arguments: Vec<(CString, CString)>,
}

/// Get the ID for this particular node.
#[no_mangle]
pub unsafe extern "C" fn rune_node_metadata_id(
    node: Option<NonNull<NodeMetadata>>,
) -> u32 {
    match node {
        Some(node) => node.as_ref().id,
        None => u32::MAX,
    }
}

/// Which kind of node is this?
///
/// Some examples are `"RAW"` or `"IMAGE"`.
///
/// # Safety
///
/// The returned pointer can't outlive the `Metadata` it came from.
#[no_mangle]
pub unsafe extern "C" fn rune_node_metadata_kind(
    node: Option<NonNull<NodeMetadata>>,
) -> *const c_char {
    match node {
        Some(node) => node.as_ref().kind.as_ptr(),
        None => ptr::null(),
    }
}

/// How many arguments have been passed to this node?
#[no_mangle]
pub unsafe extern "C" fn rune_node_metadata_num_arguments(
    node: Option<NonNull<NodeMetadata>>,
) -> c_int {
    match node {
        Some(node) => node.as_ref().arguments.len() as c_int,
        None => 0,
    }
}

/// Get the name for a particular argument, or `null` if that argument doesn't
/// exist.
#[no_mangle]
pub extern "C" fn rune_node_metadata_get_argument_name(
    node: Option<NonNull<NodeMetadata>>,
    index: c_int,
) -> *const c_char {
    let node = match node {
        Some(n) => unsafe { n.as_ref() },
        None => return ptr::null(),
    };

    node.arguments
        .get(index as usize)
        .map(|(key, _)| key.as_ptr())
        .unwrap_or_else(ptr::null)
}

/// Get the value for a particular argument, or `null` if that argument doesn't
/// exist.
#[no_mangle]
pub extern "C" fn rune_node_metadata_get_argument_value(
    node: Option<NonNull<NodeMetadata>>,
    index: c_int,
) -> *const c_char {
    let node = match node {
        Some(n) => unsafe { n.as_ref() },
        None => return ptr::null(),
    };

    node.arguments
        .get(index as usize)
        .map(|(_, value)| value.as_ptr())
        .unwrap_or_else(ptr::null)
}

use std::{
    io::{Cursor, Read},
    sync::Arc,
};

use hotg_runicos_base_runtime::BaseImage;
use crate::inspect::{CustomSection, wasm_custom_sections};

pub(crate) fn load_resources_from_custom_sections(
    wasm: &[u8],
    img: &mut BaseImage,
) {
    let resources = wasm_custom_sections(wasm)
        .filter(|s| s.name == hotg_rune_codegen::RESOURCE_CUSTOM_SECTION)
        .flat_map(|CustomSection { mut data, .. }| {
            // Note: sometimes the compiler will concatenate all
            // ".rune_resource" sections into one blob.
            core::iter::from_fn(move || {
                let (name, value, rest) =
                    hotg_rune_core::inline_resource_from_bytes(data)?;
                data = rest;
                Some((name, value))
            })
        });

    for (name, value) in resources {
        log::debug!(
            "Registering the \"{}\" resource ({} bytes)",
            name,
            value.len()
        );
        let resource: Arc<[u8]> = value.into();

        img.register_resource(name, move || {
            Ok(Box::new(Cursor::new(Arc::clone(&resource)))
                as Box<dyn Read + Send + Sync + 'static>)
        });
    }
}

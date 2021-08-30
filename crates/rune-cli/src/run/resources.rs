use std::{
    io::{Cursor, Read},
    sync::Arc,
};

use hotg_runicos_base_runtime::BaseImage;
use crate::inspect::wasm_custom_sections;

pub(crate) fn load_resources_from_custom_sections(
    wasm: &[u8],
    img: &mut BaseImage,
) {
    let sections = wasm_custom_sections(wasm)
        .filter(|s| s.name == hotg_rune_codegen::RESOURCE_CUSTOM_SECTION)
        .filter_map(|s| hotg_rune_core::inline_resource_from_bytes(s.data));

    for (name, data) in sections {
        log::debug!(
            "Registering the \"{}\" resource ({} bytes)",
            name,
            data.len()
        );
        let resource: Arc<[u8]> = data.into();

        img.register_resource(name, move || {
            Ok(Box::new(Cursor::new(Arc::clone(&resource)))
                as Box<dyn Read + Send + Sync + 'static>)
        });
    }
}

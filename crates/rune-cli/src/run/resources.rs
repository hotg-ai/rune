use std::{
    fs::File,
    io::{Cursor, Read},
    sync::Arc,
};

use anyhow::{Context, Error};
use hotg_runicos_base_runtime::BaseImage;
use crate::{
    inspect::{CustomSection, wasm_custom_sections},
    run::command::{FileResource, StringResource},
};

pub(crate) fn load_from_custom_sections(
    img: &mut BaseImage,
    wasm: &[u8],
) -> Result<(), Error> {
    let custom_sections = wasm_custom_sections(wasm)
        .context("Unable to read the WebAssembly custom sections")?;

    let resources = custom_sections
        .into_iter()
        .filter(|s| {
            s.name == hotg_rune_compiler::codegen::RESOURCE_CUSTOM_SECTION
        })
        .flat_map(|CustomSection { mut data, .. }| {
            // Note: sometimes the compiler will concatenate all
            // ".rune_resource" sections into one blob.
            core::iter::from_fn(move || {
                let (name, value, rest) =
                    hotg_rune_core::decode_inline_resource(data)?;
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

    Ok(())
}

pub(crate) fn load_from_files(img: &mut BaseImage, files: &[FileResource]) {
    for resource in files {
        let path = resource.path.clone();

        log::debug!("Registering the \"{}\" file resource", resource.name,);

        img.register_resource(&resource.name, move || {
            let f = File::open(&path).with_context(|| {
                format!("Unable to open \"{}\" for reading", path.display())
            })?;
            Ok(Box::new(f) as Box<dyn Read + Send + Sync + 'static>)
        });
    }
}

pub(crate) fn load_from_strings(img: &mut BaseImage, files: &[StringResource]) {
    for resource in files {
        let value: Arc<[u8]> = resource.value.as_bytes().into();

        log::debug!(
            "Registering the \"{}\" resource ({} bytes)",
            resource.name,
            value.len()
        );

        img.register_resource(&resource.name, move || {
            Ok(Box::new(Cursor::new(Arc::clone(&value)))
                as Box<dyn Read + Send + Sync + 'static>)
        });
    }
}

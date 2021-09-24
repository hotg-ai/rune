use crate::{BuildContext, codegen::File};

#[legion::system(for_each)]
pub(crate) fn run(File { path, data }: &File, #[resource] ctx: &BuildContext) {
    let full_path = ctx.working_directory.join(path);

    if let Some(parent) = full_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            log::error!(
                "Unable to create the \"{}\" directory: {}",
                parent.display(),
                e
            );
            return;
        }
    }

    log::debug!("Writing {} bytes to \"{}\"", data.len(), full_path.display());

    if let Err(e) = std::fs::write(&full_path, data) {
        log::error!("Unable to write to \"{}\": {}", full_path.display(), e);
    }
}

use legion::systems::CommandBuffer;

use crate::codegen::File;

#[legion::system]
pub(crate) fn run(cmd: &mut CommandBuffer) {
    let rust_toolchain = crate::rust_toolchain();
    let contents = toml::to_vec(&rust_toolchain)
        .expect("We can always serialize a hard-coded TOML object");
    let file = File::new("rust-toolchain.toml", contents);

    cmd.push((file,));
}

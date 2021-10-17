use legion::systems::CommandBuffer;
use toml::Value;

use crate::codegen::File;

#[legion::system]
pub(crate) fn run(cmd: &mut CommandBuffer) {
    let rust_toolchain = rust_toolchain();
    let contents = toml::to_vec(&rust_toolchain)
        .expect("We can always serialize a hard-coded TOML object");
    let file = File::new("rust-toolchain.toml", contents);

    cmd.push((file,));
}

/// Get a copy of the `rust-toolchain.toml` file used by the Rune project
/// itself.
fn rust_toolchain() -> Value {
    toml::toml! {
        [toolchain]
        channel = "nightly-2021-10-15"
        targets = ["wasm32-unknown-unknown"]
        components = ["rustfmt"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_toolchain_file_is_always_in_sync_with_repo() {
        let original = include_str!("../../../../rust-toolchain.toml");
        let original: Value = toml::from_str(original).unwrap();

        let got = rust_toolchain();

        assert_eq!(got, original);
    }
}

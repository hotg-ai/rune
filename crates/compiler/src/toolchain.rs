use cargo_toml::Value;

/// Get a copy of the `rust-toolchain.toml` file used by the Rune project
/// itself.
pub fn rust_toolchain() -> Value {
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
        let original = include_str!("../../../rust-toolchain.toml");
        let original: Value = toml::from_str(original).unwrap();

        let got = rust_toolchain();

        assert_eq!(got, original);
    }
}

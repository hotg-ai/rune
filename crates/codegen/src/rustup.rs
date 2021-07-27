use std::path::PathBuf;

use once_cell::sync::Lazy;

/// The version of Rust that the `rune` project is pinned to.
pub static NIGHTLY_VERSION: Lazy<String> = Lazy::new(|| {
    let rust_toolchain = include_str!("../rust-toolchain.toml");
    let parsed: OverrideFile = toml::from_str(rust_toolchain).unwrap();

    parsed
        .toolchain
        .channel
        .unwrap_or_else(|| String::from("nightly"))
});

/// Serialized form of `rust-toolchain.toml`.
///
/// Copied directly from [the rustup repo](https://github.com/rust-lang/rustup/blob/5e43c1e796f56d2757026a414f23a2a32dc97584/src/config.rs#L37-L55).
#[derive(Debug, Default, serde::Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub struct OverrideFile {
    pub toolchain: ToolchainSection,
}

/// The section of the [`OverrideFile`] specifying the compiler toolchain.
#[derive(Debug, Default, serde::Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub struct ToolchainSection {
    pub channel: Option<String>,
    pub path: Option<PathBuf>,
    pub components: Option<Vec<String>>,
    pub targets: Option<Vec<String>>,
    pub profile: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_override_file() {
        let src = r#"
            [toolchain]
            channel = "nightly-2021-05-09"
            targets = ["wasm32-unknown-unknown"]
        "#;
        let should_be = OverrideFile {
            toolchain: ToolchainSection {
                channel: Some(String::from("nightly-2021-05-09")),
                targets: Some(vec![String::from("wasm32-unknown-unknown")]),
                path: None,
                components: None,
                profile: None,
            },
        };

        let got: OverrideFile = toml::from_str(src).unwrap();

        assert_eq!(got, should_be);
    }
}

use toml::{Value, ser::Error};

pub(crate) fn generate(optimized: bool) -> Result<Value, Error> {
    let target = if optimized {
        let wasm = Target {
            rustflags: vec!["-C", "link-arg=-s"],
        };
        Some(Targets {
            wasm32_unknown_unknown: Some(wasm),
        })
    } else {
        None
    };
    let config = Config {
        target,
        net: Some(Net {
            git_fetch_with_cli: true,
        }),
    };

    Value::try_from(&config)
}

#[derive(Debug, serde::Serialize)]
struct Config {
    target: Option<Targets>,
    net: Option<Net>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
struct Targets {
    wasm32_unknown_unknown: Option<Target>,
}

#[derive(Debug, serde::Serialize)]
struct Target {
    rustflags: Vec<&'static str>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
struct Net {
    git_fetch_with_cli: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_small_binaries_when_optimised() {
        let should_be = toml::toml! {
            [target.wasm32-unknown-unknown]
            rustflags = ["-C", "link-arg=-s"]

            [net]
            git-fetch-with-cli = true
        };

        let got = generate(true).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn only_git_fetch_with_cli_for_debug_builds() {
        let should_be = toml::toml! {
            [net]
            git-fetch-with-cli = true
        };

        let got = generate(false).unwrap();

        assert_eq!(got, should_be);
    }
}

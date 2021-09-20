use legion::systems::CommandBuffer;
use crate::{BuildContext, codegen::File};

/// Generate a `.cargo/config.toml` file.
#[legion::system]
pub(crate) fn run(cmd: &mut CommandBuffer, #[resource] ctx: &BuildContext) {
    let config = generate_config(ctx.optimized);
    cmd.push((config,));
}

fn generate_config(optimized: bool) -> File {
    let target = if optimized {
        Some(Targets {
            wasm32_unknown_unknown: Target {
                rustflags: &["-C", "link-arg=-s"],
            },
        })
    } else {
        None
    };

    let net = Net {
        git_fetch_with_cli: true,
    };

    let config = Config {
        target,
        net: Some(net),
    };

    let config = toml::to_vec(&config)
        .expect("We can always serialize a Config to TOML");

    File::new(".cargo/config.toml", config)
}

#[derive(Debug, serde::Serialize)]
struct Config {
    target: Option<Targets>,
    net: Option<Net>,
}

/// The `[target]` table.
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
struct Targets {
    wasm32_unknown_unknown: Target,
}

#[derive(Debug, serde::Serialize)]
struct Target {
    rustflags: &'static [&'static str],
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
struct Net {
    git_fetch_with_cli: bool,
}

#[cfg(test)]
mod tests {
    use toml::Value;

    use super::*;

    #[test]
    fn request_small_binaries_when_optimised() {
        let should_be = toml::toml! {
            [target.wasm32-unknown-unknown]
            rustflags = ["-C", "link-arg=-s"]

            [net]
            git-fetch-with-cli = true
        };

        let got = generate_config(true);

        assert_eq!(toml::from_slice::<Value>(&got.data).unwrap(), should_be);
    }

    #[test]
    fn only_git_fetch_with_cli_for_debug_builds() {
        let should_be = toml::toml! {
            [net]
            git-fetch-with-cli = true
        };

        let got = generate_config(false);

        assert_eq!(toml::from_slice::<Value>(&got.data).unwrap(), should_be);
    }
}

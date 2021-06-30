# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Change Log](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Introduced a `rune-proc-blocks` crate containing everything you need to write
  a proc block ([#190](https://github.com/hotg-ai/rune/pull/190))
- We can now visualise pipelines containing multiple inputs and outputs
  ([#186](https://github.com/hotg-ai/rune/pull/186))
- Added a `rune inspect` sub-command which lets you extract information about
  a Rune ([#183](https://github.com/hotg-ai/rune/pull/183))
  - Information includes which `rune` binary was used to compile it and the
    pipeline it contains
  - The `.rune_graph` custom section contains a JSON blob with the pipeline
    graph
  - The `.rune_version` custom section contains a JSON representation of the
  - [`BuildInfo`][build-info] from the `rune-cli` crate
- The `IMAGE` capability built into `rune run` now supports resizing
  ([#170](https://github.com/hotg-ai/rune/issues/170))

### Changed

- **(breaking change)** The YAML format now requires a `version: 1` property
  ([#194](https://github.com/hotg-ai/rune/pull/194))
- Proc blocks are now defined using a custom derive (`#[derive(ProcBlock)]`)
  which allows the `rune` command to inspect a crate and find information about
  the proc blocks it contains
- Renamed the `runic_types` crate to `rune-core` for consistency with our other
  `rune-XXX` crates

### Fixed

- Added a `Dockerfile` to the repository which will build the `rune`
  command-line tool from scratch and include all necessary dependencies
  ([#203](https://github.com/hotg-ai/rune/pull/203))

## v0.3.0 - TinyML Summit Release (2021-05-25)

### Added

- Proc blocks, capabilities, and outputs can now all have multiple inputs and
  outputs
- Introduced a more feature-rich YAML format (typically called `Runefile.yml`)
  for defining a Rune ([#140](https://github.com/hotg-ai/rune/pull/140))
- Created `rune_py`, a Python package which wraps a lot of the proc blocks
  in the Rune project so they can be used during training
- Created a docker image which wraps the `rune` command-line tool for use on
  Windows and MacOS
- Metadata about a Rune's pipeline is now included in the WebAssembly binary as
  a custom section
- New proc blocks for
  - Image normalization ([#160](https://github.com/hotg-ai/rune/pull/160))
  - Determining the top N confidence values and pairing them with their
    associated labels ([#151](https://github.com/hotg-ai/rune/pull/151))
- Allow users to use expressions as part of capability/proc block arguments
  (e.g. to do maths or use a constant defined elsewhere)
  ([#149](https://github.com/hotg-ai/rune/pull/149))

### Changed

- Moved the `model-info` sub-command out of our internal `xtask` tool and into
  the `rune` command-line tool itself (e.g. as `rune model-info ./sine.tflite`)
  so people can use it to discover information about TensorFlow Lite models
  ([#141](https://github.com/hotg-ai/rune/pull/141))

### Fixed

- Bug determining the Runefile's "current directory"
  ([#143](https://github.com/hotg-ai/rune/pull/143))
- Type names in generated Rust code can't contain hyphens
  ([#137](https://github.com/hotg-ai/rune/pull/137))

## v0.2.0

[tinyml]: https://github.com/hotg-ai/rune/releases/tag/TinyMLSummity-RC1
[build-info]: https://docs.rs/build-info-common/0.0.23/build_info_common/struct.BuildInfo.html

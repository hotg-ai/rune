# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Change Log](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

- You can now specify a model's format by setting the `format` field under
  `args` to one of `onnx`, `tensorflow`, `tensorflow-js`, or `tensorflow-lite`
  ([#367](https://github.com/hotg-ai/rune/pull/367))
  - Note that this will just ask the runtime to load a particular model, there
    is no guarantee the model format it will be supported
- Directories which are used as a model will now be embedded in the Rune as a
  zip archive
- The `rune build` command will now emit a warning when a proc block isn't given
  a version number

### Changed

- The `hotg-rune-wasmer-runtime` and `hotg-rune-wasm3-runtime` crates have been
  merged into `hotg-rune-runtime`

### Fixed

- Warnings are no longer printed multiple times during `rune build`
  ([#367](https://github.com/hotg-ai/rune/pull/367))
- Fixed a bug where proc-blocks pulled in via git weren't using the tag
  mentioned in the version specifier ([#391](https://github.com/hotg-ai/rune/pull/391))

## [0.10.0] - 2021-10-25

### Added

- You will now be able to see when each step in a Rune's pipeline is started if
  you set the `$RUST_LOG` environment variable to `debug`
  - we recommend setting `$RUST_LOG` to something like
    `debug,regalloc=warn,hotg_runicos_base_runtime=info,hotg_rune_cli=info` to
    keep the output manageable
- Gave the `hotg_rune_core::Tensor` type a way of viewing contiguous
  sub-sections of the tensor
- The `hotg_rune_core::Tensor` type now lets you get items by index directly
  instead of requiring users to go through `TensorView`

## [0.9.3] - 2021-10-17

### Added

- The `rune run` command will now emit a warning about
  [#131](https://github.com/hotg-ai/rune/issues/131)
  ([`tensorflow/tensorflow#52300](https://github.com/tensorflow/tensorflow/issues/52300)
  upstream) when doing inference on MacOS

### Changed

- Bumped the version of Rust used to compile Runes from `nightly-2021-08-14` to
  `nightly-2021-10-15`

### Fixed

- The `#[derive(ProcBlock)]` custom derive can now handle types with generic
  parameters ([#358](https://github.com/hotg-ai/rune/pull/358))

## [0.9.2] - 2021-10-11

## [0.9.1] - 2021-10-11

## [0.9.0] - 2021-10-10

### Added

- The Rune repository now contains a JSON schema that can be used to provide
  auto-complete and basic validation for a `Runefile.yml`

### Changed

- All tensors of strings will now use `Tensor<Cow<'static, str>>` as their
  tensor types ([#345](https://github.com/hotg-ai/rune/pull/345))
- Generated Runes will now pull internal crates (`hotg-rune-core`, etc.) from
  crates.io using a version that matches the `rune` compiler
  ([#353](https://github.com/hotg-ai/rune/pull/353)) instead of using the
  `nightly` tag

### Removed

- All processing blocks have been moved to the
 [`hotg-ai/proc-block`](https://github.com/hotg-ai/proc-blocks) repository
 ([#353](https://github.com/hotg-ai/rune/pull/353))

## [0.8.0] - 2021-10-05

### Added

- The `#[transform]` attribute used with the
  `#[derive(ProcBlock)]` custom derive now accepts multiple inputs and outputs
  so you can write `#[transform(inputs = ([f32; _], [u8; 1]), outputs = str)]`
- The `rune` CLI now uses [the `human_panic` crate][human_panic] to generate
  crash reports that can be included when reporting a bug
  ([#322](https://github.com/hotg-ai/rune/issues/322))
- Data written to the `SERIAL` output will be printed to `STDOUT` as lines of
  JSON ([#316](https://github.com/hotg-ai/rune/issues/316)

### Changed

- **(Breaking Change)** The `#[transform]` attribute now expects `inputs =` and
  `outputs =` instead of `input =` and `output =`
  ([#311](https://github.com/hotg-ai/rune/issues/311))
- **(Breaking Change)** Merged the `hotg-rune-syntax` and `hotg-rune-codegen`
  crates into a new `hotg-rune-compiler` crate
- We now use `librunecoral` instead of `tflite` for TensorFlow Lite inference
  ([#301](https://github.com/hotg-ai/rune/pull/301))
- The `rune` CLI no longer logs by default. You will need to set the `RUST_LOG`
  environment variable to an appropriate value if you want to see log output
  (see [the `env_logger` crate][env_logger]'s docs for more)
  ([#316](https://github.com/hotg-ai/rune/issues/316)

### Fixed

- The `rune inspect` command panicked if it couldn't find metadata that only
  exists when installing from source or GitHub Releases
  ([#307](https://github.com/hotg-ai/rune/issues/307))
- Resolved an issue where trying to read the metadata from a Rune (done as part
  of `rune run` and `rune inspect`) would enter an infinite loop when passed a
  file that doesn't contain WebAssembly
  ([#318](https://github.com/hotg-ai/rune/pull/318))
- The `rune build` command will now error with something like *Unable to find
  "asdf" to use as an input for "some_model"* when an input isn't declared
  ([#319](https://github.com/hotg-ai/rune/issues/319))

## [0.7.0] - 2021-09-07

### Added

- Users of the web runtime are now able to provide custom model handlers
  (e.g. if you want to use your own ML framework)
- Added "Resources" to the Runefile
  ([#273](https://github.com/hotg-ai/rune/pull/273),
  [#285](https://github.com/hotg-ai/rune/pull/285))
  - Resources can be loaded from disk using the `path: ./some/file.ext` syntax
  - Resources can also be specified in the Runefile itself using the
    `inline: "value"` syntax
  - A model can be bound to a resource using `model: $MY_MODEL`
  - Procedure block arguments can be set using `some_arg: $VALUE`
  - A resource can be overridden at runtime using a different version
  - The `rune` CLI can override resources with `--resource-file NAME=./file.ext`
    or `--resource-string NAME=value`
- We now fall back to using WASM3 to evaluate WebAssembly code on platforms not
  supported by Wasmer, primarily iOS
  ([#266](https://github.com/hotg-ai/rune/pull/266),
  [#267](https://github.com/hotg-ai/rune/pull/267),
  [#271](https://github.com/hotg-ai/rune/pull/271),
  [#280](https://github.com/hotg-ai/rune/pull/280),
  [#281](https://github.com/hotg-ai/rune/pull/281))
- Gave the web runtime a builder type to simplify the process of initializing
  and loading Runes ([#279](https://github.com/hotg-ai/rune/pull/279))

### Fixed

- Resolved an issue where `cargo install hotg-rune-cli` would fail due to an
  accidental backwards incompatible change in the `cargo_toml` crate
  ([#284](https://github.com/hotg-ai/rune/issues/284))

## [0.6.0] - 2021-08-23

### Added

- Moved the Web runtime into the `hotg-ai/rune` repo under the `bindings/web`
  directory and turned it into a NPM package

### Removed

- The `--capability` command-line argument (used as `rune run ./whatever.rune
  --capability image:person.png`) has been removed after emitting a warning for
  several versions ([#256](https://github.com/hotg-ai/rune/issues/256))
- The old `Runefile` format (the text-based domain-specific language, not the
  YAML version) has now been removed

## [0.5.3] - 2021-08-11

### Fixed

- Fixed the algorithm used to locate internal dependencies when installed via
  crates.io

## [0.5.2] - 2021-08-11

### Added

- Users will no longer need to manually install nightly because a
  `rust-toolchain.toml` will be copied into the generated project
- If the `LIBRUNECORAL` environment variable is set or the `--librunecoral`
  flag is provided, `rune run` will use the specified shared library for
  hardware acceleration on TPU-enabled devices

### Changed

- The tensor dimensions specified in a Runefile now need to *exactly* match the
  dimensions expected by the model. Previously users would be allowed to pass
  something like `u8[192, 192]` to a model expecting `u8[1, 192, 192, 1]`.

## [0.4.0] - 2021-07-27

### Added

- Multiple instances of the same capability can now be provided to `rune run`
  from the command line (i.e. `rune run some.rune --image first.png --image second.jpeg`)
  ([#233](https://github.com/hotg-ai/rune/pull/233))
- Capabilities in a `Runefile.yml` can now specify which source they want to
  pull data from using a `source` argument ([#223](https://github.com/hotg-ai/rune/pull/223))
- Models can have multiple inputs and outputs ([#218](https://github.com/hotg-ai/rune/pull/218))
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

- **(breaking change)** All published crates are now prefixed with `hotg-` to
  avoid a naming collision with [the Rune programming language][rune-rs] on
  crates.io ([#236](https://github.com/hotg-ai/rune/pull/236))
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

## [0.3.0] - TinyML Summit Release (2021-05-25)

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

## [0.2.1] - 2021-03-21

<!-- next-url -->
[Unreleased]: https://github.com/assert-rs/predicates-rs/compare/{{tag_name}}...HEAD
[0.10.0]: https://github.com/assert-rs/predicates-rs/compare/hotg-rune-cli-v0.9.3...{{tag_name}}
[0.9.3]: https://github.com/assert-rs/predicates-rs/compare/hotg-rune-cli-v0.9.2...hotg-rune-cli-v0.9.3
[0.9.2]: https://github.com/assert-rs/predicates-rs/compare/hotg-rune-cli-v0.9.1...hotg-rune-cli-v0.9.2
[0.9.1]: https://github.com/assert-rs/predicates-rs/compare/hotg-rune-cli-v0.9.0...hotg-rune-cli-v0.9.1
[0.9.0]: https://github.com/assert-rs/predicates-rs/compare/hotg-rune-cli-v0.8.0...hotg-rune-cli-v0.9.0
[0.8.0]: https://github.com/assert-rs/predicates-rs/compare/hotg-rune-cli-v0.7.0...hotg-rune-cli-v0.8.0
[0.7.0]: https://github.com/assert-rs/predicates-rs/compare/hotg-rune-cli-v0.6.0...hotg-rune-cli-v0.7.0
[0.6.0]: https://github.com/assert-rs/predicates-rs/compare/hotg-rune-cli-v0.5.3...hotg-rune-cli-v0.6.0
[0.5.3]: https://github.com/assert-rs/predicates-rs/compare/hotg-rune-cli-v0.5.2...hotg-rune-cli-v0.5.3
[0.5.2]: https://github.com/assert-rs/predicates-rs/compare/v0.4.0...hotg-rune-cli-v0.5.2
[0.4.0]: https://github.com/hotg-ai/rune/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/hotg-ai/rune/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/hotg-ai/rune/compare/86763cdbb0...v0.2.1

[tinyml]: https://github.com/hotg-ai/rune/releases/tag/TinyMLSummity-RC1
[build-info]: https://docs.rs/build-info-common/0.0.23/build_info_common/struct.BuildInfo.html
[rune-rs]: https://rune-rs.github.io/
[human_panic]: https://crates.io/crates/human-panic
[env_logger]: https://docs.rs/env_logger/

 # Contributing to the Rune Project

## Getting Started

The `rune` project is just another Rust project and can be compiled using
`cargo`.

```console
$ git clone https://github.com/hotg-ai/rune.git
$ cd rune
$ cargo build --workspace
$ cargo test --workspace
```

Note that we pull in several large dependencies, so the first build may take
a couple minutes.

The `rust-toolchain.toml` file will automatically make sure you use the correct
version of Rust, but you will also need to have the following dependencies
installed:

- [`bindgen`](https://github.com/rust-lang/rust-bindgen) (`cargo install bindgen`)
- [Clang and LLVM](https://releases.llvm.org/download.html)
- [CMake](https://cmake.org/download/)
- [Docker](https://docs.docker.com/get-docker/) *(Linux only)*
- [Bazel](https://docs.bazel.build/versions/main/install.html) *(Windows and MacOS)*

The project is split up into several smaller crates using [Cargo
Workspaces][workspaces], with the most important crates being:

- `crates/compiler` - The "compiler frontend" that parses and analyses Runefiles
  then compiles them to a WebAssembly module
- `crates/runtime` - Common abstractions and types used by the various Rune
  runtimes
- `crates/wasmer-runtime` - A runtime which uses `wasmer` to execute WebAssembly
  using a user-provided `Image`
- `crates/wasm3-runtime` - A runtime which uses `wasm3` to execute WebAssembly
  using a user-provided `Image`
- `crates/rune-cli` - The `rune` command-line program, used for compiling and
  running Runes
- `crates/rune-core` - Types and abstractions shared between Runes, Proc Blocks,
  Images, and Runtimes
- `crates/xtask` - A helper program for various internal tasks
- `proc-blocks/macros` - the `#[derive(ProcBlock)]` macro
- `proc-blocks/proc-blocks` - a common crate that all Proc Blocks must use
- `proc-blocks/*` - The various Rust crates that can be used as Proc Blocks
- `images/*` The various Rust crates that can be used as base images for a
  Runefile
- `bindings/native` - FFI bindings for using the Rust runtime from non-Rust
  programs
- `bindings/web` - a TypeScript package which can run Runes in the browser
- `bindings/python` - Python bindings to various proc blocks and Rune
  functionality
- `integration-tests` - Our end-to-end test suite

The actual crate names have a `rune_` prefix to signify that they are all part
of the Rune project. Cargo doesn't have a namespacing system and there are
already several `rune_XXX` crates on crates.io so we've had to add an
*additional* `hotg_` prefix to indicate the crates are related to Hammer of the
Gods (see [hotg-ai/rune#222][issue-222] for more).

## Where Code Runs

Our crate structure is complicated by the fact that there are **three**
environments that the Rune project's code will run in, with considerable
overlap, and functionality from one environment may not make sense (or flat out
not compile) for another environment.

The environments are:

- **A developer's machine:** this is where you'll build a Rune and have access
  to a full Rust toolchain.

  It typically involves the following crates:
  - `hotg_rune_core`
  - `hotg_rune_cli`
  - `hotg_rune_compiler`
- **WebAssembly:** the Rune itself is compiled to a WebAssembly module and runs
  inside a WebAssembly virtual machine. In general, this is a very constrained
  environment and the code can only interact with the outside world via the
  WebAssembly half of its "base image" (e.g. the `hotg_runicos_base_wasm`
  crate).

  It typically involves the following crates:
  - `hotg_rune_core`
  - `hotg_runicos_base_wasm`
  - `hotg_rune_proc_blocks`
  - `hotg_runecoral`
- **Host Application:** this is the program which actually wants to execute
  Runes. It is in charge of loading a Rune into memory and initialising it,
  giving the WebAssembly code access to specific parts of the host application
  (e.g. a camera feed or serial output).

  It typically involves the following
  crates:
  - `hotg_rune_core`
  - `hotg_rune_cli`
  - `hotg_runicos_base_runtime`
  - `hotg_rune_runtime`
  - `hotg_rune_wasmer_runtime`

## Integration Tests

As well as the unit tests that you would run with `cargo test`, we've developed
a test suite that runs `rune` against compiled Runes.

These tests can be found in the `integration-tests/` folder and are split up
based on how the Rune should build/run:

- `compile-pass` - the Rune should build
- `compile-fail` - The Rune should fail to build (e.g. so you can test error
  messages)
- `run-pass` - the Rune should be able to run with the specified capabilities
- `run-fail` - running the Rune should fail (due to a missing capability,
  crash, etc.)

Let's go through the process of adding an integration test that evaluates
the `microspeech` Rune and makes sure it can detect the word, "up".

The first thing to do is create a folder for it:

```console
$ mkdir -p integration-tests/run-pass/microspeech-up
$ cd integration-tests/run-pass/microspeech-up
```

Next, we need to add our Runefile. Normally, you'd write it from scratch but
because we are already using it as an example, we'll copy it from the
`microspeech-up/` directory.

```console
$ cp ../../../examples/microspeech/Runefile.yml ./Runefile.yml
```

This Rune also requires a `model.tflite`, so we'll copy that too.

```console
$ cp ../../../examples/microspeech/model.tflite ./model.tflite
```

If all we cared about was running `rune build` (i.e. a `compile-pass` or
`compile-fail` test) we could stop here. However, we are also executing the
Rune, so we need to provide it with some capabilities that will be passed to
`rune run`.

We know that each capability only works with certain file formats, so

| Capability    | Extension    |
| ------------- | ------------ |
| Image         | `png`, `jpg` |
| Sound         | `wav`        |
| Accelerometer | `csv`        |
| Random        | `rand`       |
| Raw           | `bin`        |

Say you would normally run the Rune with
`rune run ./microspeech-up.rune --sound up.wav` then just add an `up.wav` file
to the test directory.

```console
$ cp ../../../examples/microspeech/data/up/84d1e469_nohash_0.wav ./up.wav
```

By default, the only things that get checked are the exit code from `rune`.

|             | **pass**            | **fail**           |
| ----------- | ------------------- | ------------------ |
| **compile** | `rune build` passes | `rune build` fails |
| **rune**    | `rune run` passes   | `rune run` fails   |

However, with the use of `*.stderr` and `*.stdout` files we can assert that
particular strings are written to stdout/stderr. This is typically used when
checking that a Rune writes the correct data to a `SERIAL` output or you want
to make sure a specific compile error is printed when the Runefile contains a
mistake.

For now we just want to make sure the Rune correctly outputs "up", so let's
run the Rune manually and check its output:

```console
$ rune build ./Runefile.yml
$ rune run microspeech.rune --sound up.wav
{"type_name":"utf8","channel":2,"elements":["up"],"dimensions":[1]}
```

The interesting bit is the `["up"]`, so let's copy it to a new file. Let's name
it something reasonably useful like `output.stderr` (the actual name doesn't
matter as long as it has the `stderr` extension).

```console
$ cat output.stderr
["up"]
```

The integration test runner will scan a directory for `*.stderr` files and
assert that the output from `rune run` contains exactly that text.

Now the integration test is set up and we can run the test suite.

```console
$ cargo integration-tests
[2021-05-27T15:38:43.027Z INFO  rune_integration_tests] Looking for tests
[2021-05-27T15:38:43.183Z INFO  main] compile-fail/image-is-required ... ✓
[2021-05-27T15:38:43.186Z INFO  main] compile-fail/pipeline-is-required ... ✓
[2021-05-27T15:38:43.562Z INFO  main] run-fail/missing-raw-capability ... ✓
[2021-05-27T15:38:43.974Z INFO  main] run-pass/noop ... ✓
[2021-05-27T15:38:43.974Z INFO  main] run-pass/_gesture-slope ... (skip)
[2021-05-27T15:38:45.834Z INFO  main] run-pass/sine ... ✓
[2021-05-27T15:38:46.975Z INFO  main] run-pass/gesture-ring ... ✓
[2021-05-27T15:38:45.417Z INFO  main] run-pass/microspeech-right ... ✓
[2021-05-27T15:38:46.551Z INFO  main] run-pass/microspeech-left ... ✓
[2021-05-27T15:38:47.712Z INFO  main] run-pass/microspeech-down ... ✓
[2021-05-27T15:38:44.697Z INFO  main] run-pass/microspeech-up ... ✓
[2021-05-27T15:38:48.057Z INFO  main] compile-pass/noop ... ✓
```

If you look carefully, you'll see `run-pass/microspeech-up ... ✓` indicating
that our test passed.

As a sanity check, let's modify `output.stderr` to make sure the test is actually
doing something.

```console
$ cargo integration-tests
...

[2021-10-09T14:47:32.848Z ERROR integration_tests] run-pass/microspeech-up ... ✗
[2021-10-09T14:47:32.848Z ERROR integration_tests] Unable to find the expected output in stderr.
  Expected:
  	something-else

  Actual:
  	{"type_name":"utf8","channel":2,"elements":["up"],"dimensions":[1]}
```

## Common Tasks

We use [`cargo xtask`][xtask] and cargo aliases to help with various things
during development.

The `cargo rune` alias will run a command using the `rune` binary in release
mode. Invoking the alias will also compile the binary if necessary.

```console
$ cargo rune --version
```

The best way to see what `cargo xtask` does is by using the `--help` flag:

```console
$ cargo xtask --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `/home/michael/Documents/hotg-ai/rune/target/debug/xtask --help`
xtask 0.3.0

USAGE:
    xtask <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    check-manifests    Check all Cargo.toml files are
    dist               Generate a release bundle
    help               Prints this message or the help of the given subcommand(s)
    update-schema      Update the JSON schema for a Runefile
```

## Release Process

The majority of the process for releasing a new version of Rune is automated,
but a human still needs to do be involved.

1. Create a new *"Updates from the Tinyverse"* post and go around the team
   asking people to update it with whatever cool stuff they've been doing
2. Make sure `CHANGELOG.md` is up to date
3. Use `cargo release` to bump version numbers and publish to crates.io
4. Wait for the (automatically triggered) release build to complete then move
   the associated release on GitHub Releases from "draft" to "published"
5. Use [the semver trick][semver-trick] on crates typically imported by proc
   blocks (typically `hotg-rune-core` and `hotg-rune-proc-blocks`) so existing
   proc blocks will work transparently with the new version of Rune
6. Update the various proc blocks in [`hotg-ai/proc-blocks`][hotg-proc-blocks]
   to use the latest version of dependencies then tag that using the release's
   version number

The `cargo release` step can occasionally fail, requiring you to complete it
manually.

The full process:

1. Update the headings in `CHANGELOG.md` to associate the items under
   `## Unreleased` with a version particular version and release date (e.g.
   `## [0.9.0] - 2021-10-10`). This will also add a link people to the diff
   between this release and the previous one
2. Bump the version numbers for all crates that don't have `publish = false`
   and commit the version number bump
3. Run `cargo publish` on all crates being released
4. Create a **signed** tag pointing to this commit with an appropriate tag name
   (e.g. `v0.9.0`)
5. Push the new commits and tag up to GitHub

Once the main package has been released, make sure to bump the version numbers
for any bindings and publish them to their respective package.

### Nightly Releases

We've [set up GitHub Actions][nightly-yml] to generate a "nightly" build
every 24 hours.

This will run `cargo xtask dist` on several architectures and upload new
release bundles to [*Nightly Release*][nightly-release] on GitHub Releases.

You can also trigger a nightly build manually by navigating to the
[*Nightly Workflow* page][nightly-workflow] and hitting the *"Run Workflow"*
button. See [*Manually running a workflow*][manual-workflow] for more.

[xtask]: https://github.com/matklad/cargo-xtask
[nightly-release]: https://github.com/hotg-ai/rune/releases/tag/nightly
[nightly-yml]: ./github/workflows/nightly.yml
[nightly-workflow]: https://github.com/hotg-ai/rune/actions/workflows/nightly.yml
[manual-workflow]: https://docs.github.com/en/actions/managing-workflow-runs/manually-running-a-workflow
[workspaces]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
[issue-222]: https://github.com/hotg-ai/rune/issues/222
[cargo-release]: https://crates.io/crates/cargo-release
[semver-trick]: https://github.com/dtolnay/semver-trick
[hotg-proc-blocks]: https://github.com/hotg-ai/proc-blocks

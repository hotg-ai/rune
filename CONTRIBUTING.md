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

The project is split up into several smaller crates using [Cargo
Workspaces][workspaces], with the main crates being:

- `syntax` - The "compiler frontend" that parses and analyses Runefiles
- `codegen` - A crate that generates a Rust project that gets compiled as
  WebAssembly to make up a Rune
- `runtime` - Common abstractions and types used by the various Rune runtimes
- `wasmer-runtime` - A runtime which uses `wasmer` to execute WebAssembly using
  a user-provided `Image`
- `web-runtime` - A runtime intended to be used inside a web browser
- `rune` - The `rune` command-line program, used for compiling and running
  Runes
- `runic-types` - Types shared between Runes, Proc Blocks, Images, and Runtimes
- `proc_blocks/*` - The various Rust crates that can be used as Proc Blocks
- `images/*` The various Rust crates that can be used as base images for a
  Runefile
- `xtask` - A helper program for various internal tasks
- `ffi` - FFI bindings for using `wasmer-runtime` from non-Rust programs

## Common Tasks

We use [`cargo xtask`][xtask] and cargo aliases to help with various things
during development.

The `cargo rune` alias will run a command using the `rune` binary in release
mode. This will also compile the binary, so don't be surprised if the command
seems to hang for a couple seconds on the first run.

```console
$ cargo rune --version
```

The `cargo xtask install-pre-commit-hook` command installs a git pre-commit
hook that will automatically run `rustfmt` whenever you make a commit.

The `cargo xtask dist` command generates a release bundle with all the
resources most people will need when getting started. Some things it includes
are:

- The `rune` binary compiled in release mode
- A header file and pre-compiled libraries (both static and dynamic) to allow
  the Rune runtime to be linked into a non-Rust application
- For each of the examples:
  - The compiled Rune
  - The original Runefile and TensorFlow Lite models
  - Any other data needed to train the model
  - A copy of the Rust code that is generated as part of `rune build`
- Python bindings for the various non-trivial proc blocks (for use in training)
- Other documentation that may be needed when using the project (README,
  license, etc.)

```console
$ RUST_LOG=xtask=info cargo xtask dist
    Finished release [optimized + debuginfo] target(s) in 0.09s
     Running `target/release/xtask dist`
[2021-04-09T15:07:05Z INFO  xtask::dist] Generating release artifacts
[2021-04-09T15:07:05Z INFO  xtask::dist] Compiling the `rune` binary
[2021-04-09T15:07:41Z INFO  xtask::dist] Compiling the "person_detection" rune
    Finished release [optimized + debuginfo] target(s) in 0.10s
     Running `target/release/rune build ./examples/person_detection/Runefile
        --cache-dir ./target/dist/examples/person_detection/rust
        --output ./target/dist/examples/person_detection/person_detection.rune`
[2021-04-09T15:07:59Z INFO  xtask::dist] Copying example artifacts across
...
[2021-04-09T15:08:47Z INFO  xtask::dist] Generating Python bindings to the proc blocks
🔗 Found pyo3 bindings
🐍 Found CPython 3.9 at python3.9
    Finished release [optimized] target(s) in 0.05s
📦 Built wheel for CPython 3.9 to ./target/dist/wheels/proc_blocks-0.1.0-cp39-cp39-linux_x86_64.whl
[2021-04-09T15:08:49Z INFO  xtask::dist] Writing the release archive to "./target/rune.x86_64-unknown-linux-gnu.zip"
```

## Continuous Deployment

We've [set up GitHub Actions][nightly-yml] to generate a "nightly" build
every 24 hours.

This will run `cargo xtask dist` on several architectures and upload new
release bundles to [*Nightly Release*][nightly-release] on GitHub Releases.

You can also trigger a nightly build manually by navigating to the
[*Nightly Workflow* page][nightly-workflow] and hitting the *"Run Workflow"*
button. See [*Manually running a workflow*][manual-workflow] for more.

## Benchmarks

The `runtime/` directory contains several benchmarks for comparing things like
how long it takes to build a rune, startup times for the `Runtime`, and
execution time.

Benchmarks are done using [`criterion`][criterion] and you will need to install
their command-line helper, `cargo criterion`.

```console
$ cargo install cargo-criterion
```

Once that is done you can `cd` to the `runtime/` directory and run the
benchmarks.

```console
$ cd runtime
$ cargo criterion
   Compiling rune-runtime v0.1.0 (/home/michael/Documents/hotg-ai/rune/runtime)
    Finished bench [optimized] target(s) in 4.37s

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

execute-sine/optimised-rune
                        time:   [4.7431 us 4.8673 us 4.9952 us]
execute-sine/debug-rune time:   [51.182 us 51.500 us 51.910 us]
execute-sine/no-rune    time:   [17.482 ns 17.643 ns 17.840 ns]

startup/sine            time:   [7.3093 ms 7.3262 ms 7.3439 ms]
startup/gesture         time:   [8.5877 ms 8.6171 ms 8.6487 ms]
```

benchmark reports will be available inside the repo's `target/` directory.

```console
$ ls ../target/criterion/*
../target/criterion/data:
main

../target/criterion/reports:
execute-sine  index.html  startup
```

[xtask]: https://github.com/matklad/cargo-xtask
[criterion]: https://bheisler.github.io/criterion.rs/book/criterion_rs.html
[nightly-release]: https://github.com/hotg-ai/rune/releases/tag/nightly
[nightly-yml]: ./github/workflows/nightly.yml
[nightly-workflow]: https://github.com/hotg-ai/rune/actions/workflows/nightly.yml
[manual-workflow]: https://docs.github.com/en/actions/managing-workflow-runs/manually-running-a-workflow
[workspaces]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

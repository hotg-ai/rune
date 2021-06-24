# Rune

[![Continuous integration](https://github.com/hotg-ai/rune/actions/workflows/main.yml/badge.svg)](https://github.com/hotg-ai/rune/actions/workflows/main.yml)
![Total Downloads](https://img.shields.io/github/downloads/hotg-ai/rune/total.svg)

**[Nightly Release][nightly] | [API Docs][api-docs]**

Rune is a containerization technology for deploying TinyML applications to
extremely constraint devices.

## Quickstart

### Dependencies

You can compile and install the latest version of the `rune` command-line
tool from source using `cargo`.

```console
$ cargo install --git https://github.com/hotg-ai/rune rune
    Updating git repository `https://github.com/hotg-ai/rune`
  Installing rune v0.2.1 (https://github.com/hotg-ai/rune#0beff26f)
    Updating crates.io index
    ...
   Compiling rune v0.2.1 (~/.cargo/git/checkouts/rune-005ac9981b4d47b5/0beff26/rune)
    Finished dev [unoptimized + debuginfo] target(s) in 4m 49s
  Installing ~/.cargo/bin/rune
   Installed package `rune v0.2.1 (https://github.com/hotg-ai/rune#0beff26f)` (executable `rune`)
```

**Pre-compiled binaries are also available on GitHub Releases under the
[Nightly Release][nightly].**

The `rune` tool turns a Runefile into Rust code so it can be compiled to
WebAssembly, so you will also need to have [the Rust compiler][rustup]
installed.

```console
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

A Rune runs in a constrained environment and doesn't have access to the
standard library, so to implement our own allocation error handler we'll need
to use the nightly compiler.

```console
$ rustup toolchain install nightly
```

Compiling to WebAssembly requires copies of Rust's core libraries
cross-compiled to `wasm32-unknown-unknown`.

```console
$ rustup target add wasm32-unknown-unknown
```

### Compiling a Rune

A Runefile is a declarative way to define a ML pipeline. How Runefiles and
Runes fit together is explained in detail in [*What's In A
Rune?*][whats-in-a-rune], but here's a typical example:

```
FROM runicos/base

CAPABILITY<U8[96, 96]> image IMAGE

PROC_BLOCK<U8[96, 96],f32[96, 96]> normalize hotg-ai/rune#proc_blocks/normalize

MODEL<U8[96, 96],U8[3]> person_detection ./model.tflite

PROC_BLOCK<U8[3],UTF8> label hotg-ai/rune#proc_blocks/person_detection_agg \
   --labels=unknown,person,not_person

OUT SERIAL

RUN image person_detection label serial
```

The `rune` command-line program can then compile this to a Rune.

```console
$ rune build ./examples/person_detection/Runefile
[2021-04-09T17:09:18.312Z DEBUG rune::build] Parsing "./examples/person_detection/Runefile"
[2021-04-09T17:09:18.312Z DEBUG rune::build] Compiling person_detection in "/home/michael/.cache/runes/person_detection"
[2021-04-09T17:09:18.313Z DEBUG rune_codegen] Executing "cargo" "+nightly" "build" "--target=wasm32-unknown-unknown" "--quiet" "--release"
[2021-04-09T17:09:31.656Z DEBUG rune::build] Generated 267829 bytes
```

### Running a Rune

You can run a Rune locally using the `rune run` command.

```console
$ rune run ./examples/sine/sine.rune
[2021-04-09T17:41:17.940Z INFO  rune::run] Running rune: ./examples/sine/sine.rune
[2021-04-09T17:41:17.956Z DEBUG rune_wasmer_runtime] Loading image
[2021-04-09T17:41:17.957Z DEBUG rune_wasmer_runtime] Instantiating the WebAssembly module
[2021-04-09T17:41:17.957Z DEBUG rune_wasmer_runtime] Loaded the Rune
[2021-04-09T17:41:17.957Z INFO  rune::run] Call 0
[2021-04-09T17:41:17.957Z DEBUG rune_wasmer_runtime] Running the rune
[2021-04-09T17:41:17.957Z INFO  runicos_base::image] Serial: [4.8617184e-2]
```

This will fail for any Rune that expects input from the outside world, though.
For example, the `person_detection` Rune expects to be provided with an image
(presumably one containing a person).

```console
$ rune run ./examples/person_detection/person_detection.rune
[2021-04-09T17:42:18.996Z INFO  rune::run] Running rune: ./examples/person_detection/person_detection.rune
[2021-04-09T17:42:19.006Z DEBUG rune_wasmer_runtime] Loading image
[2021-04-09T17:42:19.006Z DEBUG rune_wasmer_runtime] Instantiating the WebAssembly module
Error: Unable to initialize the virtual machine

Caused by:
    0: Unable to call the _manifest function
    1: RuntimeError: The image capability is not supported
    2: The image capability is not supported
```

You use the `--capability` argument to provide capabilities. The syntax is
`--capability $type:$value`, where `$type` is one of the supported capability
types (e.g. `random`, `accel`, `sound`, `image`) and the `$value` is a value
specific to that capability, typically the name of an input file.

```console
$ rune run ./examples/person_detection/person_detection.rune \
   --capability image:examples/person_detection/image_grayscale.png
[2021-04-09T17:45:27.489Z INFO  rune::run] Running rune: ./examples/person_detection/person_detection.rune
[2021-04-09T17:45:27.489Z DEBUG rune::run] Loading an image from "examples/person_detection/image_grayscale.png"
[2021-04-09T17:45:27.504Z DEBUG rune_wasmer_runtime] Loading image
[2021-04-09T17:45:27.504Z DEBUG rune_wasmer_runtime] Instantiating the WebAssembly module
[2021-04-09T17:45:27.505Z DEBUG rune_wasmer_runtime] Loaded the Rune
[2021-04-09T17:45:27.505Z DEBUG rune_wasmer_runtime] Running the rune
[2021-04-09T17:45:27.510Z INFO  runicos_base::image] Serial: "person"
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE.md) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT.md) or
   http://opensource.org/licenses/MIT)

at your option.

It is recommended to always use [cargo-crev][crev] to verify the
trustworthiness of each of your dependencies, including this one.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

The intent of this crate is to be free of soundness bugs. The developers will
do their best to avoid them, and welcome help in analysing and fixing them.

[crev]: https://github.com/crev-dev/cargo-crev
[nightly]: https://github.com/hotg-ai/rune/releases/tag/nightly
[api-docs]: https://hotg-ai.github.io/rune/
[rustup]: https://rustup.rs/
[whats-in-a-rune]: https://tinyverse.substack.com/p/whats-in-a-rune

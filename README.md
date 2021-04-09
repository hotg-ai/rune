# Rune

[![Continuous integration](https://github.com/hotg-ai/rune/actions/workflows/main.yml/badge.svg)](https://github.com/hotg-ai/rune/actions/workflows/main.yml)

**[Nightly Release][nightly] | [API Docs][api-docs]**

Rune is a containerization technology for deploying TinyML applications to
extremely constraint devices.

## Getting Started

To start building your own Runes you will first need:

- [The Rust compiler](https://rustup.rs/)
- The WebAssembly target (`rustup target add wasm32-unknown-unknown`)
- The `rune` command-line tool

### Runefile

A `Runefile` is similar to `Dockerfile` in that it is a text document that
defines capabilities, processing blocks, feature transformation, models and
model outputs to assemble the `Rune`.

A simplistic example of this is would be:

*TODO: refine the below with an exact working version*

```
FROM runicos/base

CAPABILITY AUDIO audio --hz 16000 --samples 150 --sample-size 1500

PROC_BLOCK runicos/fft fft

MODEL ./example.tflite model --input [150,1] --output 1

RUN audio fft model

OUT serial
```

In this example a audio with fft (fast fourier transformation) block can be run with the model.

## Building and Running this project

- Install Rust from [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)
- Build the project with `cargo build`
- This should create Rune executable in `./target/debug/rune`
- Run the project with `cargo run`

[nightly]: https://github.com/hotg-ai/rune/releases/tag/nightly
[api-docs]: https://hotg-ai.github.io/rune/

# Rune

[![Continuous integration](https://github.com/hotg-ai/rune/actions/workflows/main.yml/badge.svg)](https://github.com/hotg-ai/rune/actions/workflows/main.yml)

<table>
   <thead>
      <tr>
         <th>Release</th>
         <th>Download</th>
      </tr>
   </thead>
   <tbody>
      <tr>
         <td>Nightly</td>
         <td>
            <li><a href="https://storage.cloud.google.com/rune-registry.appspot.com/nightly/rune.x86_64-unknown-linux-gnu.zip?authuser=1">
               Linux (<code>x86_64-unknown-linux-gnu</code>)
            </a></li>
            <li>Windows (<code>x86_64-pc-windows-msvc</code>)</li>
            <li>Mac (<code>x86_64-apple-darwin</code>)</li>
            <li>iOS (<code>x86_64-apple-ios</code>)</li>
         </td>
         </tr>
   </tbody>
</table>

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

### Usage

#### Build

Using the `rune` cli you can build containers
that are tagged and available.

*List available containers*

`rune container ls`

*Build new containers*

`rune build .`

*Run the containers locally simulated*

`rune exec ${CONTAINER-ID}`


## Building and Running this project

- Install Rust from [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)
- Build the project with `cargo build`
- This should create Rune executable in `./target/debug/rune`
- Run the project with `cargo run`


## Private Git Repos

To get deps from our private git repos we need to
use `ssh agent`.

Add the below to your `.ssh/config`
```
Host github.com
   UseKeychain yes
   AddKeysToAgent yes
   IdentityFile ~/.ssh/id_rsa
```

and run:
`ssh-add -K ~/.ssh/id_rsa`

## Developing

We use [`cargo xtask`][xtask] to help with various things during development.

You can use `cargo xtask install-pre-commit-hook` to install a pre-commit hook
that will automatically run `rustfmt` whenever you make a commit.

The `cargo rune` alias can be used to run the `rune` binary (e.g.
`cargo rune run ./sine.rune` or `cargo rune build ./Runefile`).

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


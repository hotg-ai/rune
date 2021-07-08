# Build Instructions for the Rune Native Library

The `librune_native` library is designed to provide as many platforms as
possible with a way to execute Runes. Rust's strong cross-compilation story
plays a big part in this.

There are two parts to the `librune_native` library, the pre-compiled binaries
(both static and dynamic libraries), and a header file specifying what is
available in each library.

We use [the `cross` tool][cross] to generate both parts of the library, so make
sure it's installed.

```console
$ cargo install cross
```

`cross` works by running the compiler in a Docker container which already has
the appropriate C compiler toolchain installed, so make sure `docker` is
running.

Keep in mind that certain functionality aren't available on certain targets,
typically because a dependency (e.g. TensorFlow) can't cross-compile. We use
[Cargo Features][features] to selectively enable parts of the public API, and
the same list of features will need to be used when generating both the library
and header file.

## Cross-Compiling the Library

Cross-compiling the main library is simple enough. You just need to specify the
target and list of enabled features.

For example, if cross-compiling to `aarch64-linux-android` (64-bit Android) with
the `wasmer-runtime` feature (`tflite` doesn't cross-compile) then you would
compile the `rune-native` crate with something like this:

```console
$ cross build --target aarch64-linux-android \
              --package rune-native \
              --features wasmer-runtime \
              --release
$ ls -l target/aarch64-linux-android/release
drwxr-xr-x    - michael  8 Jul 17:24 build
drwxr-xr-x    - michael  8 Jul 17:55 deps
drwxr-xr-x    - michael  8 Jul 17:24 examples
drwxr-xr-x    - michael  8 Jul 17:24 incremental
.rw-r--r--  65M michael  8 Jul 17:55 librune_native.a
.rw-r--r-- 1.6k michael  8 Jul 17:29 librune_native.d
.rwxr-xr-x  11M michael  8 Jul 17:55 librune_native.so
```

## Header Files

We use [the `safer_ffi` crate][safer-ffi] to provide more high-level FFI
bindings, and the crate comes with its own mechanism for generating header
files.

The header generation works by registering each exported type and function
using a procedural macro, then executing a piece of code which iterates through
each registered item so we can generate the equivalent C declarations.

Unfortunately, you can't cross-compile the library to the target architecture
and do the header generation on the host because there is no guarantee the
target and host bindings will be identical. Instead, we can instruct `cross` to
run the header generator inside a `qemu` virtual machine.

Continuing with the `aarch64-linux-android` architecture and its single
`wasmer-runtime` feature, we just need to run `cross test` and enable the
additional `c-headers` feature.

```console
$ cross test --target aarch64-linux-android \
              --package rune-native \
              --features wasmer-runtime \
              --features c-headers \
              --release \
              -- \
            generate_headers
$ head target/rune.h
/** \file
 * rune-native v0.1.0
 *
 * Authors: The Rune Developers
 * License: MIT OR Apache 2.0
 * Commit: ef93b88418f75441e962cef9478aedccaa5851db
 * Compiler: rustc 1.54.0-nightly (1c6868aa2 2021-05-27)
 * Target: aarch64-linux-android
 * Enabled Features: wasmer-runtime
 *
 * Native bindings to the `rune` project.
 */
```

[cross]: https://github.com/rust-embedded/cross
[features]: https://doc.rust-lang.org/cargo/reference/features.html
[safer-ffi]: https://github.com/getditto/safer_ffi

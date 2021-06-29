# Rune Native Bindings

A Rust library which can be used by native code.

## Cross-Compiling to Android

First you'll need to install Android Studio and the following components:

* Android SDK Tools
* NDK
* CMake
* LLDB

We then need to set some environment variables pointing to the newly installed
files.

```console
$ export ANDROID_HOME=$HOME/Android/Sdk
$ export NDK_HOME=$ANDROID_HOME/ndk/22.1.7171670
```

(your specific path may vary)

Next we need to tell `cargo` which binaries to use when compiling and linking by
updating our `~/.cargo/config`.

These can all be found in the `$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/`
folder.

```toml
# ~/.cargo/config
[target.aarch64-linux-android]
ar = "$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar"
linker = "$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang"
```

(Note: you will need to expand the `$NDK_HOME` yourself)

The project also has a couple C dependencies so we need to set some more
environment variables to let the `cc` crate find `ar` and `clang`.

```console
$ export AR_aarch64_linux_android=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar
$ export CC_aarch64_linux_android=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang
```

From here, you should just be able to do a `cargo build`, specifying the desired
feature flags.

```console
$ cargo build --target aarch64-linux-android --no-default-features --features wasmer-runtime
$ ls ../../target/debug/
librune_native.a
librune_native.so
```

## Header Files

You can generate header files for these bindings using the following command:

```console
$ cargo test --package rune-native --features c-headers -- generate_headers
$ head ../../target/rune.h
 /** \file
 * rune-native v0.1.0

 * Authors: The Rune Developers
 * License: MIT OR Apache 2.0
 * Commit: db0ba625551e30f4a46fb7d6b3765e7bd17a6937
 * Compiler: rustc 1.54.0-nightly (1c6868aa2 2021-05-27)
 * Enabled Features: default, rune-wasmer-runtime, tflite, wasmer-runtime
 *
 * Native bindings to the `rune` project.
```

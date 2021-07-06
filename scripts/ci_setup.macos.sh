#!/bin/sh

# We need to update rustup because the mac version is out of date and
# self-update is disabled. https://github.com/rust-lang/rustup/issues/2766
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain none -y
# Make sure rust is on our path
source ~/.cargo/env

# and now we can do the linux setup like normal
exec "$(git rev-parse --show-toplevel)/ci_setup.linux.sh"

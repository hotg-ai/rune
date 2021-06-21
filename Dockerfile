FROM debian:latest

RUN apt-get update -y && apt-get -y install libclang-dev curl build-essential git
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
# Putting the toolchain file in / means we always use the right rustc version
COPY rust-toolchain.toml /rust-toolchain.toml
COPY . /app/

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup-init.sh && \
    sh rustup-init.sh --default-toolchain none -y && \
    rustup component add rustfmt && \
    rustup show && \
    cargo fetch && \
    # Install Rune
    cargo install --root / --path /app/rune --debug && \
    rune --version --verbose && \
    # Delete any bulky dependencies installed with Rune
    rm -rf target $CARGO_HOME/git $CARGO_HOME/registry && \
    rustup component add rustfmt && \
    cargo fmt --version && \
    # Build a rune to prime the cargo cache for building a real Rune
    rune build /app/examples/microspeech/Runefile.yml --cache-dir /tmp/rune --output /tmp/rune/microspeech.rune && \
    # And remove everything we compiled
    rm -rf /tmp/rune && \
    # And refresh our app directory
    rm -rf /app && mkdir /app

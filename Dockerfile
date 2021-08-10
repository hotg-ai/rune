FROM rust:1.53 as build

RUN apt-get update -y && apt-get -y install libclang-dev clang curl build-essential git 

WORKDIR /app
# Putting the toolchain file in / means we always use the right rustc version
COPY rust-toolchain.toml /rust-toolchain.toml
COPY . /app/

RUN rustup show && \
    cargo fetch && \
    # Install Rune
    cargo install --root / --path /app/crates/rune-cli --locked --verbose && \
    rune version --verbose && \
    # Delete any bulky dependencies installed with Rune
    rm -rf target $CARGO_HOME/git $CARGO_HOME/registry


FROM debian:latest

WORKDIR /app
COPY --from=build /bin/rune /usr/local/bin/rune
# Putting the toolchain file in / means we always use the right rustc version
COPY rust-toolchain.toml /rust-toolchain.toml
ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt-get update -y && \
    apt-get install -y curl build-essential && \
    rm -rf /var/lib/apt/lists/* && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup-init.sh && \
    sh rustup-init.sh --default-toolchain none -y && \
    rustup component add rustfmt && \
    rustup show

CMD [ "rune" ]

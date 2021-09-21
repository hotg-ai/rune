FROM rust:1.53 as build

RUN apt-get update -y && apt-get -y install libclang-dev clang curl build-essential git python3-numpy zip unzip curl wget cmake pkg-config

ARG BAZEL_VERSION=4.0.0
RUN wget -O /bazel https://github.com/bazelbuild/bazel/releases/download/${BAZEL_VERSION}/bazel-${BAZEL_VERSION}-installer-linux-x86_64.sh && \
    bash /bazel && \
    rm -f /bazel

WORKDIR /lib
RUN git clone --recurse-submodules https://github.com/hotg-ai/librunecoral && cd librunecoral && \
    mkdir -p dist/include && install runecoral/runecoral.h dist/include && \
    bazel --batch build -c opt --linkopt=-Wl,--strip-all --config linux_x86_64 //runecoral:runecoral && \
    mkdir -p dist/lib/linux/x86_64 && install bazel-bin/runecoral/librunecoral.a dist/lib/linux/x86_64

ENV RUNECORAL_DIST_DIR /lib/librunecoral/dist

WORKDIR /app
# Putting the toolchain file in / means we always use the right rustc version
COPY rust-toolchain.toml /rust-toolchain.toml
COPY . /app/

RUN rustup show && \
    cargo fetch && \
    cargo install --debug bindgen && \
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

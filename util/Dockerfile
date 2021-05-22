FROM rust:latest as builder

WORKDIR /app

RUN apt-get update -y

RUN apt-get install unzip

RUN curl -L https://github.com/hotg-ai/rune/releases/download/nightly/rune.x86_64-unknown-linux-gnu.zip \
    -o rune.x86_64-unknown-linux-gnu.zip
    

RUN ls

RUN unzip rune.x86_64-unknown-linux-gnu.zip

RUN chmod a+x rune

FROM rust:latest

WORKDIR /app

RUN rustup toolchain install nightly-2021-05-09

RUN rustup target add wasm32-unknown-unknown --toolchain nightly-2021-05-09

COPY --from=builder /app/rune /usr/local/bin


#CMD ["sh"]

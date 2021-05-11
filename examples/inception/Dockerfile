FROM rust:latest as builder

WORKDIR /app

RUN apt-get update -y

RUN apt-get install unzip

RUN curl -L https://github.com/hotg-ai/rune/releases/download/nightly/rune.x86_64-unknown-linux-gnu.zip \
    -o rune.x86_64-unknown-linux-gnu.zip

RUN ls

RUN unzip rune.x86_64-unknown-linux-gnu.zip

ADD ./Runefile .

ADD download_model.sh .

RUN sh download_model.sh

RUN chmod a+x rune


RUN rustup toolchain install nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

RUN ./rune build ./Runefile 

FROM ubuntu:latest

WORKDIR /app

COPY --from=builder /app/app.rune .

COPY --from=builder /app/rune .



#CMD ["sh"]
FROM ubuntu:18.04

RUN apt-get update && apt-get install -y curl
RUN apt-get install build-essential -y

# RUN mkdir -p /user/turreta-rust-builder/src
RUN mkdir -p /user/compiler-book/src
# WORKDIR /user/turreta-rust-builder/src
WORKDIR /user/compiler-book/src

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY Cargo.toml Cargo.toml
COPY ./src ./src

RUN cargo build --release

RUN cargo install --path .
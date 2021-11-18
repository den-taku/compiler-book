FROM ubuntu:latest

# Install some
RUN apt-get update && apt-get install -y curl
RUN apt-get install build-essential -y

# working directory is compier-book
RUN mkdir -p /user/compiler-book
WORKDIR /user/compiler-book

# download rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY Cargo.toml Cargo.toml

# for caching
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release

# copy testcase
RUN mkdir testcase
COPY ./testcase ./testcase

# copy code
COPY ./src ./src

RUN rm -f target/release/deps/compiler_book*

RUN cargo test
RUN cargo build --release
RUN cargo run --release
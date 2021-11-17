#~/usr/bin.bash

cargo fmt

cargo c

cargo test

docker build -t compilerbook .

docker run compilerbook cargo run --release

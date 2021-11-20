#~/usr/bin.bash

cargo fmt

cargo c

cargo test

docker build -t compilerbook .

docker run --rm compilerbook cargo run --release '23 - 8+5- 3'
docker run --rm compilerbook cargo run --release '23 - 8+5- 3 9 +'

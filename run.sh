#~/usr/bin.bash

cargo fmt

cargo c

cargo test

docker build -t compilerbook .

docker run --rm compilerbook cargo run --release '((4 + 3) / 7 + 4) * (4 - 2)'
docker run --rm compilerbook cargo run --release '23 * 8+5/ 3 9 +'
docker run --rm compilerbook cargo run --release '((4 + 3) * 5 - 4 * 5'

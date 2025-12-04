#!/usr/bin/env bash

# https://gist.github.com/mohanpedala/1e2ff5661761d3abd0385e8223e16425
set -euo pipefail

echo "Which day? (01, 02, ...)"
read -r day

echo "Which part? (1 or 2)"
read -r part

echo "Which benchmarking?"
echo "1 hyperfine"
echo "2 internal"
read -r type

case "${type}" in
  "1")
    cargo build --release --bin "day${day}" --no-default-features --features "part${part}"
    hyperfine --warmup 10 --time-unit microsecond --shell none "target/release/day${day}"
    ;;
  "2")
    cargo run --release --bin "day${day}" --no-default-features --features "benchmark,part${part}"
    ;;
  *)
    echo "Unknown benchmarking type '${type}'!" >&2
    ;;
esac

# Advent Of Code 2025

https://github.com/ThiFerNe/aoc-2025

By ThiFerNe

With "Advent Of Code 2025" you can see my hand-crafted solutions to
the [Advent of Code 2025](https://adventofcode.com/2025).

You are free to copy, modify, and distribute "Advent Of Code 2025" with attribution under the terms of the MIT License.
See the [LICENSE](./LICENSE) file for details.

## Run The Project

Before running "Advent Of Code 2025" you need:

- Rust Toolchain, see <https://rust-lang.org/tools/install/> (currently using `1.91.1`)

Each day is its own binary and can be run like `cargo run --bin day01`.
Running only a select part of the puzzle is achieved through features like
`cargo run --bin day01 --no-default-features --features part1` or `part2`.

For the full speed use `--release` after `cargo run` like `cargo run --release --bin day01`.

Internal timings will be given when also adding the `internal_timings` feature, which is enabled on default.

When giving `benchmark` as feature an internal benchmark is being done.

---

*README.md created with the help
of [ddbeck's readme-checklist](https://github.com/ddbeck/readme-checklist/blob/eb0ac8ce9733c4f6e4bc552fbb2c3db60561554a/checklist.md).*

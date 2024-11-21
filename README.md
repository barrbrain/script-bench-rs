# Rust scripting languages benchmark

The project goal is to benchmark most popular embedded scripting languages for Rust.

- [boa](https://boajs.dev)
- [mlua](https://crates.io/crates/mlua) (Lua 5.4 and Luau)
- [rhai](https://crates.io/crates/rhai)
- [rquickjs](https://crates.io/crates/rquickjs)
- [rune](https://crates.io/crates/rune)

The benchmark is designed to cover not only the performance of code evaluation but interoperability with Rust too.

## Getting your own results

Simply run the `bench.py` script to generate images. It requires `cargo criterion` and `python3-matplotlib` package installed.

## Environment

|          |                               |
|----------|-------------------------------|
| OS       | Ubuntu 22.04, m6i.16xlarge    |
| boa      | v0.19.1                       |
| mlua     | v0.10.1                       |
| rhai     | v1.20.0                       |
| rquickjs | v0.8.1                        |
| rune     | 496577c95e6e082342aac0e6e71c6 |
| rustc    | v1.81.0                       |

## Results

![Sort Rust objects](Sort%20Rust%20objects.png)

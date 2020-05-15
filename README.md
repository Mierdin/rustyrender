# rustyrender

A simple, CPU-bound graphics renderer for learning purposes

# Lessons

These will closely follow the [tinyrenderer](https://github.com/ssloy/tinyrenderer/wiki) source on the basics of computer graphics and rendering, with the obvious exception that the original exercises will be translated from C++ to Rust.

## Lesson 1 - Bresenham's Line Drawing Algorithm

**Lesson guide**: [here](https://github.com/ssloy/tinyrenderer/wiki/Lesson-1-Bresenham%E2%80%99s-Line-Drawing-Algorithm)

Run example:

```bash
cargo run --example lesson1
```

# Misc

## Profiling

[Profiling](https://rust-lang.github.io/packed_simd/perf-guide/prof/linux.html) this code can be done easily with the `perf` tool. Debug info is currently enabled for dev (non release) builds. Compile this project and run the corresponding binary like so (as root):

```
perf record --call-graph=lbr target/debug/examples/lesson1
perf report --hierarchy -M intel
```

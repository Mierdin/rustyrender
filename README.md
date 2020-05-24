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


## Saved for Later

```rust
// TODO(mierdin): it wasn't enough to provide ImageBuffer, we had to provide the typs after as well. Why?
// https://stackoverflow.com/questions/35488820/how-to-create-a-rust-struct-with-an-imageimagebuffer-as-a-member
// Also, I originally had no return type, which meant that anything after this function call lost ownership of imgbuf. Had to return it to pass back ownership.
pub fn line(v0: Vec2f, v1: Vec2f, color: Rgb<u8>, mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>>{

    debug!("Writing line from {},{} to {},{}", v0.x, v0.y, v1.x, v1.y);

    // Using width since we're only expecting square dimensions
    for t in 0..imgbuf.width() {
        let t = t as f32 * (1.0 / imgbuf.width() as f32);
        let x = v0.x + (v1.x - v0.x) * t;
        let y = v0.y + (v1.y - v0.y) * t;

        imgbuf.put_pixel(x as u32, y as u32, color);
    }
    imgbuf
}
```
#[macro_use] extern crate log;
extern crate image;
extern crate num_complex;
extern crate tobj;

use image::{ImageBuffer, Rgb};

pub const WHITE: Rgb<u8> = image::Rgb([255, 255, 255]);
pub const BLACK: Rgb<u8> = image::Rgb([0, 0, 0]);
pub const RED: Rgb<u8> = image::Rgb([255, 0, 0]);
pub const BLUE: Rgb<u8> = image::Rgb([0, 0, 255]);

// Creating a single value for controlling scale. This sets the dimensions of the imagebuffer,
// but is also used to determine how often to draw a pixel, and for how long
pub const SCALE: u32 = 1000;

pub struct Vec2f {
    x: f32,
    y: f32
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Vec2f {
        Vec2f { x: x, y: y }
    }
}

// TODO(mierdin): it wasn't enough to provide ImageBuffer, we had to provide the typs after as well. Why?
// https://stackoverflow.com/questions/35488820/how-to-create-a-rust-struct-with-an-imageimagebuffer-as-a-member
// Also, I originally had no return type, which meant that anything after this function call lost ownership of imgbuf. Had to return it to pass back ownership.
pub fn line(v0: Vec2f, v1: Vec2f, color: Rgb<u8>, mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>>{

    debug!("Writing line from {},{} to {},{}", v0.x, v0.y, v1.x, v1.y);

    for t in 0..SCALE {
        let t = t as f32 * (1.0 / SCALE as f32);
        let x = v0.x + (v1.x - v0.x) * t;
        let y = v0.y + (v1.y - v0.y) * t;

        imgbuf.put_pixel(x as u32, y as u32, color);
    }

    imgbuf

}

// Writes all pixels in an image buffer with the same color
pub fn background(color: Rgb<u8>, mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = color;
    }

    imgbuf
}

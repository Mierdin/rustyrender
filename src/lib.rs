#[macro_use] extern crate log;
extern crate image;
extern crate num_complex;
extern crate tobj;

use image::{ImageBuffer, Rgb};

pub const WHITE: Rgb<u8> = image::Rgb([255, 255, 255]);
pub const BLACK: Rgb<u8> = image::Rgb([0, 0, 0]);
pub const RED: Rgb<u8> = image::Rgb([255, 0, 0]);
pub const GREEN: Rgb<u8> = image::Rgb([0, 255, 0]);
pub const BLUE: Rgb<u8> = image::Rgb([0, 0, 255]);


// Writes all pixels in an image buffer with the same color
pub fn background(color: Rgb<u8>, mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = color;
    }

    imgbuf
}

#[macro_use] extern crate log;
extern crate image;
extern crate num_complex;
extern crate tobj;

use image::GenericImageView;
use image::{ImageBuffer, Pixel, Rgb};
use std::fs::File;
use std::io::BufReader;

const WHITE: Rgb<u8> = image::Rgb([255, 255, 255]);
const BLACK: Rgb<u8> = image::Rgb([0, 0, 0]);
const RED: Rgb<u8> = image::Rgb([255, 0, 0]);
const BLUE: Rgb<u8> = image::Rgb([0, 0, 255]);

// Creating a single value for controlling scale. This sets the dimensions of the imagebuffer,
// but is also used to determine how often to draw a pixel, and for how long
const SCALE: u32 = 1000;

fn main() {

    let (models, materials) = tobj::load_obj("african_head.obj", false).expect("Failed to load file");

    debug!("# of models: {}", models.len());
    debug!("# of materials: {}", materials.len());

    let mesh = &models[0].mesh;

    let mut imgbuf = image::ImageBuffer::new(SCALE+1, SCALE+1);
    imgbuf = background(BLACK, imgbuf);

    // num_face_indices is just a vector which stores the number of indices used by each face.
    // We can iterate over this to figure out how many indices we should include in the slice of mesh.indices
    let mut next_face = 0;
    for f in 0..mesh.num_face_indices.len() {

        let end = next_face + mesh.num_face_indices[f] as usize;

        // face_indices is a vector containing the index for three vertices that make up a face
        let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
        debug!("    face[{}] = {:?}", f, face_indices);

        // Loop through the three sides of the face
        for j in 0..3 {

            // Get the index for the two verticies for this side of the face
            let v0_idx = *face_indices[j] as usize; // = 0
            let v1_idx = *face_indices[(j+1)%3] as usize; // = 1
            debug!("v0_idx={}, v1_idx={}", v0_idx, v1_idx);

            // Retrieve raw positions from OBJ file. We're also negating each position to be compatible with our imagebuffer
            let v0_x_raw = -mesh.positions[3 * v0_idx];
            let v0_y_raw = -mesh.positions[3 * v0_idx + 1];
            let v1_x_raw = -mesh.positions[3 * v1_idx];
            let v1_y_raw = -mesh.positions[3 * v1_idx + 1];
            debug!("v0_x_raw={}, v0_y_raw={}, v1_x_raw={}, v1_y_raw={}", v0_x_raw, v0_y_raw, v1_x_raw, v1_y_raw);

            // Determine x/y for each end of the line, taking scale into account
            let v0x = (v0_x_raw + 1.0) * SCALE as f32 / 2.0;
            let v0y = (v0_y_raw + 1.0) * SCALE as f32 / 2.0;
            let v1x = (v1_x_raw + 1.0) * SCALE as f32 / 2.0;
            let v1y = (v1_y_raw + 1.0) * SCALE as f32 / 2.0;
            debug!("v0x={}, v0y={}, v1x={}, v1y={}", v0x, v0y, v1x, v1y);

            imgbuf = line(v0x, v0y, v1x, v1y, WHITE, imgbuf);

        }
        next_face = end;
    }
    imgbuf.save("head.png").unwrap();

}

// This was my first test of the line() function, by passing in explicit coordinates to the function call.
fn simple_lines() {

    let mut imgbuf = image::ImageBuffer::new(SCALE, SCALE);

    imgbuf = background(WHITE, imgbuf);

    // TODO(mierdin): Currently there is nothing preventing us from passing in a value > SCALE
    // and panicking, here, or in the obj draw code above
    imgbuf = line(20.0, 20.0, 20.0, 840.0, BLACK, imgbuf);
    imgbuf = line(500.0, 304.0, 890.0, 40.0, RED, imgbuf);
    imgbuf = line(50.0, 50.0, 800.0, 800.0, BLUE, imgbuf);

    imgbuf.save("line.png").unwrap();

}

// TODO(mierdin): it wasn't enough to provide ImageBuffer, we had to provide the typs after as well. Why?
// https://stackoverflow.com/questions/35488820/how-to-create-a-rust-struct-with-an-imageimagebuffer-as-a-member
// Also, I originally had no return type, which meant that anything after this function call lost ownership of imgbuf. Had to return it to pass back ownership.
fn line(x0: f32, y0: f32, x1: f32, y1: f32, color: Rgb<u8>, mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>>{

    debug!("Writing line from {},{} to {},{}", x0, y0, x1, y1);

    for t in 0..SCALE {
        let t = t as f32 * (1.0 / SCALE as f32);
        let x = x0 + (x1 - x0) * t;
        let y = y0 + (y1 - y0) * t;

        imgbuf.put_pixel(x as u32, y as u32, color);
    }

    imgbuf

}

// Writes all pixels in an image buffer with the same color
fn background(color: Rgb<u8>, mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = color;
    }

    imgbuf
}

#[macro_use] extern crate log;
extern crate image;
extern crate num_complex;
extern crate tobj;
extern crate rustyrender;

use image::GenericImageView;
use image::{ImageBuffer, Pixel, Rgb};
use std::fs::File;
use std::io::BufReader;
use rustyrender::{line, background, SCALE, WHITE, RED, BLACK, BLUE, Vec2f};

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

            let v0 = Vec2f::new(v0x, v0y);
            let v1 = Vec2f::new(v1x, v1y);

            imgbuf = line(v0, v1, WHITE, imgbuf);

        }
        next_face = end;
    }
    imgbuf.save("head.png").unwrap();

}

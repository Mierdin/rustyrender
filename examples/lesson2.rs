#[macro_use] extern crate log;
extern crate image;
extern crate num_complex;
extern crate tobj;
extern crate rustyrender;
extern crate rand;

use rand::Rng;

use std::mem;
use std::cmp;

use image::GenericImageView;
use image::{ImageBuffer, Pixel, Rgb, imageops};
use std::fs::File;
use std::io::BufReader;

use rustyrender::{SCALE, WHITE, RED, BLACK, BLUE, GREEN};
use rustyrender::{Vec2f, Vec3f};
use rustyrender::{line, background, normalize, triangle, barycentric};

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

        let mut screen_coords: Vec<Vec2f> = Vec::with_capacity(3);
        let mut world_coords: Vec<Vec3f> = Vec::with_capacity(3);

        // Loop through the three sides of the face
        for j in 0..3 {

            let v = Vec3f::new(
                -mesh.positions[3 * *face_indices[j] as usize],
                -mesh.positions[3 * (*face_indices[j] as usize)+1],
                -mesh.positions[3 * (*face_indices[j] as usize)+2]
            );
            world_coords.push(v);

            // TODO(mierdin): Dynamically retrieve width/height (ran into borrowing)
            // screen_coords[j] = Vec2f::new((world_coords.x+1.0)*200 as f32/2.0, (world_coords.y+1.0)*200 as f32/2.);
            screen_coords.push(Vec2f::new((v.x+1.0)*SCALE as f32/2.0, (v.y+1.0)*SCALE as f32/2.));


        }


        // The normal to the triangle can be calculated simply as the cross product of its two sides.
        let n = (world_coords[2]-world_coords[0]).cross(world_coords[1]-world_coords[0]);
        // We then have to set the magnitude of this vector to 1.
        let n = normalize(n);

        let light_dir = Vec3f::new(0.,0.,-1.);

        // the intensity of illumination is equal to the scalar product (aka dot product) of the light vector
        // and the normal to the given triangle.
        let intensity = n.dot(light_dir);
        if  intensity > 0. {
            // TODO(mierdin): This originally had four numbers - Rgba?
            let color: Rgb<u8> = image::Rgb([(intensity*255.0) as u8, (intensity*255.0) as u8, (intensity*255.0) as u8]);
            imgbuf = triangle(&screen_coords, color, imgbuf); 
        }

        next_face = end;
    }





    // imgbuf = triangle(pts, RED, imgbuf); 
    // imgbuf = imageops::flip_vertical(&imgbuf);
    imgbuf.save("head_fill.png").unwrap();
}


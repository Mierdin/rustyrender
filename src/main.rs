#[macro_use] extern crate log;
extern crate image;
extern crate num_complex;
extern crate tobj;

mod geometry;

use image::{Rgb, imageops};

use geometry::types::{Vec3f};
use geometry::Triangle;

/*

TODO

- Update functions to borrow properly so you don't have to return imgbuf
- Fix the stupid constant for dimensions, and instead, have the functions retrieve from imgbuf
*/


fn main() {
    let scale = 1000;

    // MODEL
    // let diffuse = image::open("obj/african_head_diffuse.tga").unwrap().to_rgb();
    let (models, materials) = tobj::load_obj("obj/african_head.obj", false).expect("Failed to load file");
    let mesh = &models[0].mesh;
    debug!("# of models: {}", models.len());
    debug!("# of materials: {}", materials.len());

    // IMAGE
    let mut imgbuf = image::ImageBuffer::new(scale+1, scale+1);
    let mut zbuffer = vec![-1.; (imgbuf.width()*imgbuf.height()) as usize];


    // TRIANGLE
    // TODO - Need to create a new Triangle object that we instantiate below. This would probably have a property like "scale"
    // that you can pass in on creation
    // num_face_indices is just a vector which stores the number of indices used by each face.
    // We can iterate over this to figure out how many indices we should include in the slice of mesh.indices
    let mut next_face = 0;
    for f in 0..mesh.num_face_indices.len() {

        let end = next_face + mesh.num_face_indices[f] as usize;

        // face_indices is a vector containing the index for three vertices that make up a face
        // such that face_indices[0] is the index within mesh.positions for the first of three vertices for the triangle
        let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
        debug!("    face[{}] = {:?}", f, face_indices);

        let mut screen_coords: Vec<Vec3f> = Vec::with_capacity(3); // Used for rasterization
        let mut world_coords: Vec<Vec3f> = Vec::with_capacity(3);  // Used for 3d calculations like light intensity

        // Loop through the three sides of the face
        for j in 0..3 {
            let v = Vec3f::new(
                mesh.positions[3 * *face_indices[j] as usize],
                mesh.positions[3 * (*face_indices[j] as usize)+1],
                mesh.positions[3 * (*face_indices[j] as usize)+2]
            );
            world_coords.push(v);

            screen_coords.push(v.to_screen(scale as f32));
        }

        let mut t = Triangle{
            v0: world_coords[0],
            v1: world_coords[1],
            v2: world_coords[2],
            scale: scale as f32,

            imgbuf: &mut imgbuf,
            zbuffer: &mut zbuffer
        };
        t.render();

        next_face = end;
    }

    imgbuf = imageops::flip_vertical(&imgbuf);
    imgbuf.save("lesson3.png").unwrap();
}

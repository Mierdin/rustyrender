#[macro_use] extern crate log;
extern crate image;
extern crate num_complex;
extern crate tobj;
extern crate rustyrender;

use image::Rgb;

use rustyrender::{SCALE, BLACK};
use rustyrender::{Vec2f, Vec3f};
use rustyrender::{background, triangle};

fn main() {
    let (models, materials) = tobj::load_obj("african_head.obj", false).expect("Failed to load file");

    debug!("# of models: {}", models.len());
    debug!("# of materials: {}", materials.len());

    let mesh = &models[0].mesh;

    let mut imgbuf = image::ImageBuffer::new(SCALE+1, SCALE+1);
    imgbuf = background(BLACK, imgbuf);

    let light_dir = Vec3f::new(0.,0.,-1.);

    // num_face_indices is just a vector which stores the number of indices used by each face.
    // We can iterate over this to figure out how many indices we should include in the slice of mesh.indices
    let mut next_face = 0;
    for f in 0..mesh.num_face_indices.len() {

        let end = next_face + mesh.num_face_indices[f] as usize;

        // face_indices is a vector containing the index for three vertices that make up a face
        let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
        debug!("    face[{}] = {:?}", f, face_indices);

        let mut screen_coords: Vec<Vec2f> = Vec::with_capacity(3); // Used for rasterization
        let mut world_coords: Vec<Vec3f> = Vec::with_capacity(3);  // Used for 3d calculations like light intensity

        // Loop through the three sides of the face
        for j in 0..3 {
            let v = Vec3f::new(
                -mesh.positions[3 * *face_indices[j] as usize],
                -mesh.positions[3 * (*face_indices[j] as usize)+1],
                -mesh.positions[3 * (*face_indices[j] as usize)+2]
            );
            world_coords.push(v);

            screen_coords.push(Vec2f::new(
                ((v.x+1.0)*SCALE as f32/2.0).round(),  // Rounding screen coordinates to remove gaps between edges
                ((v.y+1.0)*SCALE as f32/2.0).round()
            ));
        }

        // To determine which way a face is pointed, we need to get it's normal vector.
        // This can be calculated by getting the cross product of two of its sides.
        let mut n = (world_coords[2]-world_coords[0]).cross(world_coords[1]-world_coords[0]);

        // We also need this vector to be "normalized", which is to set its magnitude to 1
        n.normalize();

        // Next, we calculate the intensity of illumination for this face. This can be derived via
        // the scalar product (aka dot product) of the light vector and the normal to the given triangle (n).
        // I am multiplying this by 0.75 to bring the overall brightness down a bit - this is just a personal preference.
        let intensity = n.dot(light_dir) * 0.85;
        if  intensity > 0. {
            let color: Rgb<u8> = image::Rgb([(intensity*255.0) as u8, (intensity*255.0) as u8, (intensity*255.0) as u8]);
            imgbuf = triangle(&screen_coords, color, imgbuf); 
        }

        next_face = end;
    }

    // imgbuf = imageops::flip_vertical(&imgbuf);
    imgbuf.save("head_fill.png").unwrap();
}


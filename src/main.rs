#[macro_use] extern crate log;
extern crate image;
extern crate num_complex;
extern crate tobj;
// extern crate rustyrender;

use image::{Rgb, imageops};

use rustyrender::{Vec3f};
use rustyrender::{triangle};

/*

TODO

- Update functions to borrow properly so you don't have to return imgbuf
- Fix the stupid constant for dimensions, and instead, have the functions retrieve from imgbuf
*/


fn main() {
    let scale = 1000;
    let light_dir = Vec3f::new(0.,0.,-1.);

    let (models, materials) = tobj::load_obj("obj/african_head.obj", false).expect("Failed to load file");
    let mesh = &models[0].mesh;
    debug!("# of models: {}", models.len());
    debug!("# of materials: {}", materials.len());

    let mut imgbuf = image::ImageBuffer::new(scale+1, scale+1);

    // TODO(mierdin): Pre-populating with a positive 1.0 here because this seems to be the lower-bound 
    // for z-coordinates during the z-buffer check. You should see if you've accidentally reversed the Z-axis somewhere.
    let mut zbuffer = vec![1.; (imgbuf.width()*imgbuf.height()) as usize];

    // TODO - Need to create a new Triangle object that we instantiate below. This would probably have a property like "scale"
    // that you can pass in on creation

    // num_face_indices is just a vector which stores the number of indices used by each face.
    // We can iterate over this to figure out how many indices we should include in the slice of mesh.indices
    let mut next_face = 0;
    for f in 0..mesh.num_face_indices.len() {

        let end = next_face + mesh.num_face_indices[f] as usize;

        // face_indices is a vector containing the index for three vertices that make up a face
        let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
        debug!("    face[{}] = {:?}", f, face_indices);

        let mut screen_coords: Vec<Vec3f> = Vec::with_capacity(3); // Used for rasterization
        let mut world_coords: Vec<Vec3f> = Vec::with_capacity(3);  // Used for 3d calculations like light intensity


        // Loop through the three sides of the face
        for j in 0..3 {
            let v = Vec3f::new(
                -mesh.positions[3 * *face_indices[j] as usize],
                -mesh.positions[3 * (*face_indices[j] as usize)+1],
                -mesh.positions[3 * (*face_indices[j] as usize)+2]
            );
            world_coords.push(v);

            // TODO(mierdin): put into world2screen function
            screen_coords.push(Vec3f::new(
                ((v.x+1.0)*scale as f32/2.0).round(),  // Rounding screen coordinates to remove gaps between edges
                ((v.y+1.0)*scale as f32/2.0).round(),
                v.z
            ));
        }

        // To determine which way a face is pointed, we need to get its normal vector.
        // This can be calculated by getting the cross product of two of its sides.
        let mut n = (world_coords[2]-world_coords[0]).cross(world_coords[1]-world_coords[0]);

        // We also need this vector to be "normalized", which is to set its magnitude to 1
        n.normalize();

        // mesh.texcoords // This is the vector where the textures are stored. Use the index from before

        // Next, we calculate the intensity of illumination for this face. This can be derived via
        // the scalar product (aka dot product) of the light vector and the normal to the given triangle (n).
        // I am multiplying this by a fraction to bring the overall brightness down a bit - this is just a personal preference.
        let intensity = n.dot(light_dir) * 0.85;
        if  intensity > 0. {
            let color: Rgb<u8> = image::Rgb([(intensity*255.0) as u8, (intensity*255.0) as u8, (intensity*255.0) as u8]);
            imgbuf = triangle(&screen_coords, &mut zbuffer, color, imgbuf); 
        }
        next_face = end;
    }

    // imgbuf = imageops::flip_vertical(&imgbuf);
    imgbuf.save("lesson3.png").unwrap();
}

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
use rustyrender::{line, background, SCALE, WHITE, RED, BLACK, BLUE, GREEN, Vec2f, Vec3f};

fn main() {
    // let mut imgbuf = image::ImageBuffer::new(1000, 1000);
    // imgbuf = background(BLACK, imgbuf);
    // let pts: Vec<Vec2f> = vec![Vec2f::new(10.0, 10.0), Vec2f::new(100.0, 30.0), Vec2f::new(190.0, 160.0)];


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

        // Loop through the three sides of the face
        for j in 0..3 {

            let world_coords = Vec3f::new(
                -mesh.positions[3 * *face_indices[j] as usize],
                -mesh.positions[3 * (*face_indices[j] as usize)+1],
                -mesh.positions[3 * (*face_indices[j] as usize)+2]
            );

            // TODO(mierdin): Dynamically retrieve width/height (ran into borrowing)
            // screen_coords[j] = Vec2f::new((world_coords.x+1.0)*200 as f32/2.0, (world_coords.y+1.0)*200 as f32/2.);
            screen_coords.push(Vec2f::new((world_coords.x+1.0)*1000 as f32/2.0, (world_coords.y+1.0)*1000 as f32/2.));


        }

        let mut rng = rand::thread_rng();
        let rndcolor: Rgb<u8> = image::Rgb([rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255)]);
        println!("COLOR - {:?}", rndcolor);
        imgbuf = triangle(&screen_coords, rndcolor, imgbuf);

        // for (int i=0; i<model->nfaces(); i++) { 
        //     std::vector<int> face = model->face(i); 
        //     Vec2i screen_coords[3]; 

        //     Loop through the three vertices in the face
        //     for (int j=0; j<3; j++) { 
        //         Vec3f world_coords = model->vert(face[j]); 
        //         screen_coords[j] = Vec2i((world_coords.x+1.)*width/2., (world_coords.y+1.)*height/2.); 
        //     } 
        //     triangle(screen_coords[0], screen_coords[1], screen_coords[2], image, TGAColor(rand()%255, rand()%255, rand()%255, 255)); 
        // }

        next_face = end;
    }





    // imgbuf = triangle(pts, RED, imgbuf); 
    // imgbuf = imageops::flip_vertical(&imgbuf);
    imgbuf.save("head_fill.png").unwrap();
}


// This is the first method taught in lesson 2. I didn't quite get there, and it seems this is the less preferable
// approach anyways. Will move on to barycentric approach, and perhaps revisit this if there's value and I have time.
pub fn triangle_enumerate_lines(mut v0: Vec2f, mut v1: Vec2f, mut v2: Vec2f, color: Rgb<u8>, mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {

    imgbuf = line(v0, v1, WHITE, imgbuf); 
    imgbuf = line(v1, v2, WHITE, imgbuf); 
    imgbuf = line(v2, v0, WHITE, imgbuf); 

    // sort the vertices, t0, t1, t2 lower−to−upper (bubblesort yay!)
    if v0.y>v1.y { mem::swap(&mut v0, &mut v1) };
    if v0.y>v2.y { mem::swap(&mut v0, &mut v2) }; 
    if v1.y>v2.y { mem::swap(&mut v1, &mut v2) };

    // imgbuf = line(v0, v1, BLUE, imgbuf); 
    // imgbuf = line(v1, v2, BLUE, imgbuf); 
    // imgbuf = line(v2, v0, BLUE, imgbuf); 


    println!("###############################################");
    println!("#############   NEW TRIANGLE   ################");
    println!("###############################################");

    let total_height = v2.y-v0.y;  // Largest Y minus smallest Y
    let mut y = v0.y;  // start at the bottom

    // Go up until we reach the middle Y
    while y <= v1.y {
        let segment_height = v1.y-v0.y+1.0;
        println!("--------------------------------------");
        println!("y | {}", y);
        println!("v0x {} | v0y {} | v1x {} | v1y {}", v0.x, v0.y, v1.x, v1.y);
        println!("total_height {} | segment_height {}", total_height, segment_height);
        let alpha = (y-v0.y)/total_height;
        let beta = (y-v0.y)/segment_height; // be careful with divisions by zero 

        println!("alpha {} | beta {}", alpha, beta);

        // My attempt
        // TODO(Mierdin): figure out how to implement multiply for Vec2f so you don't have to do this separately here
        let A: Vec2f = v0 + (v2-v0);
        let B: Vec2f = v0 + (v1-v0);
        println!("FIRST | Ax {} | Ay {} | Bx {} | By {}", A.x, A.y, B.x, B.y);
        let A = Vec2f::new(A.x*alpha, A.y);
        let B = Vec2f::new(B.x*beta, B.y);
        // Original C++
        // Vec2i A = t0 + (t2-t0)*alpha; 
        // Vec2i B = t0 + (t1-t0)*beta; 

        println!("SECOND | Ax {} | Ay {} | Bx {} | By {}", A.x, A.y, B.x, B.y);

        imgbuf.put_pixel(A.x as u32, y as u32, RED);
        imgbuf.put_pixel(B.x as u32, y as u32, GREEN);

        y += 1.0;
    }

    imgbuf
}

fn cross(v1: Vec3f, v2: Vec3f) -> Vec3f {
    return Vec3f::new(v1.y*v2.z - v1.z*v2.y, v1.z*v2.x - v1.x*v2.z, v1.x*v2.y - v1.y*v2.x)
}

fn barycentric(pts: &Vec<Vec2f>, P: &Vec2f) -> Vec3f {
    let u = cross(Vec3f::new(pts[2].x-pts[0].x, pts[1].x-pts[0].x, pts[0].x-P.x), Vec3f::new(pts[2].y-pts[0].y, pts[1].y-pts[0].y, pts[0].y-P.y));

    /* `pts` and `P` has integer value as coordinates
       so `abs(u[2])` < 1 means `u[2]` is 0, that means
       triangle is degenerate, in this case return something with negative coordinates */
    // TODO(mierdin): This is the original statement from the C++ code. This **may** not be needed, but it still works, so /shrug
    if u.z.abs()<1.0 { return Vec3f::new(-1.0,1.0,1.0) };
    let ret = Vec3f::new(1.0-(u.x+u.y)/u.z, u.y/u.z, u.x/u.z);
    // println!("Barycentric is returning: {:?}", ret);

    ret
}


fn triangle(pts: &Vec<Vec2f>, color: Rgb<u8>, mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {

    let mut bboxmin = Vec2f::new((imgbuf.width()-1) as f32, (imgbuf.height()-1) as f32);
    let mut bboxmax = Vec2f::new(0.0, 0.0);
    let clamp = Vec2f::new((imgbuf.width()-1) as f32, (imgbuf.height()-1) as f32);

    for i in 0..3 {
        bboxmin.x = (0.0 as f32).max(bboxmin.x.min(pts[i].x)); 
        bboxmax.x = clamp.x.min(bboxmax.x.max(pts[i].x)); 

        bboxmin.y = (0.0 as f32).max(bboxmin.y.min(pts[i].y)); 
        bboxmax.y = clamp.y.min(bboxmax.y.max(pts[i].y)); 
    }

    let mut P = Vec2f::new(bboxmin.x, bboxmin.y);
    // println!("bboxmin.x {} | bboxmin.y {}", bboxmin.x, bboxmin.y);
    // println!("bboxmax.x {} | bboxmax.y {}", bboxmax.x, bboxmax.y);
    while P.x <= bboxmax.x {
        // println!("P.x {}", P.x);

        // Reset P.y before each loop
        P.y = bboxmin.y;
        while P.y <= bboxmax.y {
            // println!("P.y {}", P.y);
            let bc_screen = barycentric(&pts, &P);
            if bc_screen.x<0.0 || bc_screen.y<0.0 || bc_screen.z<0.0 {
                P.y += 1.0;
                continue
            }; 
            imgbuf.put_pixel(P.x as u32, P.y as u32, color);
            P.y += 1.0;
        }
        P.x += 1.0;
    }

    imgbuf
}

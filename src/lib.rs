#[macro_use] extern crate log;
extern crate image;
extern crate num_complex;
extern crate tobj;
use std::mem::swap;

use std::ops::{Sub, Add};

use image::{ImageBuffer, Rgb};

pub const WHITE: Rgb<u8> = image::Rgb([255, 255, 255]);
pub const BLACK: Rgb<u8> = image::Rgb([0, 0, 0]);
pub const RED: Rgb<u8> = image::Rgb([255, 0, 0]);
pub const GREEN: Rgb<u8> = image::Rgb([0, 255, 0]);
pub const BLUE: Rgb<u8> = image::Rgb([0, 0, 255]);

// Creating a single value for controlling scale. This sets the dimensions of the imagebuffer,
// but is also used to determine how often to draw a pixel, and for how long
// pub const SCALE: u32 = 1000;

#[derive(Copy, Clone, Debug)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Vec2f {
        Vec2f { x: x, y: y }
    }
}

impl Sub for Vec2f {
    type Output = Vec2f;

    fn sub(self, other: Vec2f) -> Vec2f {
        Vec2f {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Vec2f {
    type Output = Vec2f;

    fn add(self, other: Vec2f) -> Vec2f {
        Vec2f {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f { x: x, y: y, z: z }
    }

    pub fn magnitude(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();

        // http://www.fundza.com/vectors/normalize/
        self.x = self.x * (1.0 / mag);
        self.y = self.y * (1.0 / mag);
        self.z = self.z * (1.0 / mag);
    }

    // cross returns the cross product of this vector and another vector: "other".
    // http://sites.science.oregonstate.edu/math/home/programs/undergrad/CalculusQuestStudyGuides/vcalc/crossprod/crossprod.html
    pub fn cross(self, other: Vec3f) -> Vec3f {
        return Vec3f::new(self.y*other.z - self.z*other.y, self.z*other.x - self.x*other.z, self.x*other.y - self.y*other.x)
    }

    // Also referred to as a scalar product
    // https://www.mathsisfun.com/algebra/vectors-dot-product.html
    pub fn dot(self, other: Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


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

// Writes all pixels in an image buffer with the same color
pub fn background(color: Rgb<u8>, mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = color;
    }

    imgbuf
}


pub fn barycentric(pts: &Vec<Vec3f>, P: &Vec3f) -> Vec3f {
    let s0 = Vec3f::new(pts[2].x-pts[0].x, pts[1].x-pts[0].x, pts[0].x-P.x);
    let s1 = Vec3f::new(pts[2].y-pts[0].y, pts[1].y-pts[0].y, pts[0].y-P.y);
    let u = s0.cross(s1);
    /* `pts` and `P` has integer value as coordinates
       so `abs(u[2])` < 1 means `u[2]` is 0, that means
       triangle is degenerate, in this case return something with negative coordinates */
    // TODO(mierdin): This is the original statement from the C++ code. We're not using integers, so may not run into this
    // problem. However, it doesn't appear to be causing issues by leaving it in.
    if u.z.abs()<1.0 { 
        return Vec3f::new(-1.0,1.0,1.0)
    };
    Vec3f::new(1.0-(u.x+u.y)/u.z, u.y/u.z, u.x/u.z)
}

pub fn triangle(pts: &Vec<Vec3f>, zbuffer: &mut Vec<f32>, color: Rgb<u8>, mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut bboxmin = Vec2f::new((imgbuf.width()-1) as f32, (imgbuf.height()-1) as f32);
    let mut bboxmax = Vec2f::new(0.0, 0.0);
    let clamp = Vec2f::new((imgbuf.width()-1) as f32, (imgbuf.height()-1) as f32);

    for i in 0..3 {
        bboxmin.x = (0.0 as f32).max(bboxmin.x.min(pts[i].x)); 
        bboxmax.x = clamp.x.min(bboxmax.x.max(pts[i].x)); 

        bboxmin.y = (0.0 as f32).max(bboxmin.y.min(pts[i].y)); 
        bboxmax.y = clamp.y.min(bboxmax.y.max(pts[i].y)); 
    }

    let mut P = Vec3f::new(bboxmin.x, bboxmin.y, 0.);
    while P.x <= bboxmax.x {
        P.y = bboxmin.y;  // Important to reset P.y here since we're using a while loop
        while P.y <= bboxmax.y {
            let bc_screen = barycentric(&pts, &P);
            if bc_screen.x<0.0 || bc_screen.y<0.0 || bc_screen.z<0.0 {
                P.y += 1.0;
                continue
            };
            P.z = 0.;
            let z = pts[0].z * bc_screen.x + pts[1].z * bc_screen.y + pts[2].z * bc_screen.z;
            let idx = (P.x + P.y * imgbuf.width() as f32) as usize;

            // If the existing pixel has less depth than the currently proposed one
            // overwrite the respective zbuffer entry, and place the pixel
            //
            // TODO(mierdin): This is reversed from the C++ example, but this is the only way
            // I could get it working, combined with initializing the zbuffer with positive 2.0
            // This makes me think I've reversed the Z-axis somewhere.
            if zbuffer[idx] > z {
                zbuffer[idx] = z;
                imgbuf.put_pixel(P.x as u32, P.y as u32, color);
            }
            P.y += 1.0;
        }
        P.x += 1.0;
    }

    imgbuf
}

pub fn rasterize(p0: &mut Vec2f, p1: &mut Vec2f, color: Rgb<u8>, mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>>, ybuffer: &mut Vec<u32>) -> ImageBuffer<Rgb<u8>, Vec<u8>>{
    if p0.x>p1.x {
        swap(p0, p1);
    }

    let mut x = p0.x;
    while x <= p1.x {
        let t = (x-p0.x)/(p1.x-p0.x) as f32;
        let y = p0.y*(1.-t) + p1.y*t;
        println!("x - {}", x);
        println!("p1.x - {}", p1.x);
        println!("len - {}", ybuffer.len());
        if ybuffer[x as usize] < y as u32 {
            ybuffer[x as usize] = y as u32;
            // imgbuf.put_pixel(x as u32, 0, color);
            for j in 0..16 {
                imgbuf.put_pixel(x as u32, j, color);
            }
        }

        x += 1.;
    }

    imgbuf
}

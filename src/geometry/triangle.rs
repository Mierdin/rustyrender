

use crate::geometry::types::{Vec2f, Vec3f};
use image::{ImageBuffer, Rgb};


// TODO(Mierdin): create constructor function and remove pub from fields
pub struct Triangle<'a> {
    pub v0: Vec3f,
    pub v1: Vec3f,
    pub v2: Vec3f,
    pub scale: f32,

    // THESE in particular are sensitive, in that they're potentially shared. Maybe lifetimes can help?
    pub imgbuf: &'a mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub zbuffer: &'a mut Vec<f32>
}

impl <'a> Triangle<'a> {

    pub fn render(&mut self) {
        let light_dir = Vec3f::new(0.,0.,-1.);
        // Need a render function for each triangle that:
        /*
            - gets intensity
            - gets color (textures)
            - converts world coords to screen coords
            - calls draw function
        */

        // GET INTENSITY (method for what type though?)
        // To determine which way a face is pointed, we need to get its normal vector.
        // This can be calculated by getting the cross product of two of its sides.
        let mut n = (self.v2-self.v0).cross(self.v1-self.v0);
        // We also need this vector to be "normalized", which is to set its magnitude to 1
        n.normalize();
        // Next, we calculate the intensity of illumination for this face. This can be derived via
        // the scalar product (aka dot product) of the light vector and the normal to the given triangle (n).
        // I am multiplying this by a fraction to bring the overall brightness down a bit - this is just a personal preference.
        let intensity = n.dot(light_dir) * 0.85;
        if intensity > 0. {
            let color: Rgb<u8> = image::Rgb([(intensity*255.0 as f32) as u8, (intensity*255.0 as f32) as u8, (intensity*255.0 as f32) as u8]);
            self.draw(color); 
        }

    }

    // Get the barycentric coordinates of point P within this triangle
    // This must operate on screen coords, not world coords
    pub fn barycentric(&self, P: &Vec3f) -> Vec3f {
        let v0 = self.v0.to_screen(self.scale);
        let v1 = self.v1.to_screen(self.scale);
        let v2 = self.v2.to_screen(self.scale);

        let s0 = Vec3f::new(v2.x-v0.x, v1.x-v0.x, v0.x-P.x);
        let s1 = Vec3f::new(v2.y-v0.y, v1.y-v0.y, v0.y-P.y);
        let u = s0.cross(s1);
        if u.z.abs()<1.0 { 
            return Vec3f::new(-1.0,1.0,1.0)
        };
        Vec3f::new(1.0-(u.x+u.y)/u.z, u.y/u.z, u.x/u.z)
    }

    pub fn draw(&mut self, color: Rgb<u8>) {

        let mut screen_coords: Vec<Vec3f> = Vec::with_capacity(3); // Used for rasterization
        screen_coords.push(self.v0.to_screen(self.scale));
        screen_coords.push(self.v1.to_screen(self.scale));
        screen_coords.push(self.v2.to_screen(self.scale));

        // Set up bounding box to greatly reduce the pixels we'll need to iterate over in order
        // to draw the triangle
        let mut bboxmin = Vec2f::new((self.imgbuf.width()-1) as f32, (self.imgbuf.height()-1) as f32);
        let mut bboxmax = Vec2f::new(0.0, 0.0);
        let clamp = Vec2f::new((self.imgbuf.width()-1) as f32, (self.imgbuf.height()-1) as f32);
        for i in 0..3 {
          bboxmin.x = (0.0 as f32).max(bboxmin.x.min(screen_coords[i].x)); 
          bboxmax.x = clamp.x.min(bboxmax.x.max(screen_coords[i].x)); 
          bboxmin.y = (0.0 as f32).max(bboxmin.y.min(screen_coords[i].y)); 
          bboxmax.y = clamp.y.min(bboxmax.y.max(screen_coords[i].y));
        }

        let mut P = Vec3f::new(bboxmin.x, bboxmin.y, 0.);
        while P.x <= bboxmax.x {
            P.y = bboxmin.y;  // Important to reset P.y here since we're using a while loop
            while P.y <= bboxmax.y {
                let bc_screen = self.barycentric(&P);
                if bc_screen.x<0.0 || bc_screen.y<0.0 || bc_screen.z<0.0 {
                    P.y += 1.0;
                    continue
                };
                P.z = 0.;
                let z = screen_coords[0].z * bc_screen.x + screen_coords[1].z * bc_screen.y + screen_coords[2].z * bc_screen.z;
                let idx = (P.x + P.y * self.imgbuf.width() as f32) as usize;

                // If the existing pixel has less depth than the currently proposed one
                // overwrite the respective zbuffer entry, and place the pixel
                println!("{} | {} | {:?}", P.x, P.y, color);
                if self.zbuffer[idx] < z {
                    self.zbuffer[idx] = z;
                    self.imgbuf.put_pixel(P.x as u32, P.y as u32, color);
                }
                P.y += 1.0;
            }
            P.x += 1.0;
        }
    }


}
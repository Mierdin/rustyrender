use std::mem::swap;
use std::ops::{Sub, Add};

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

    pub fn to_screen(self, scale: f32) -> Vec3f {
      Vec3f::new(
          ((self.x+1.0)*scale/2.0).round(),  // Rounding screen coordinates to remove gaps between edges
          ((self.y+1.0)*scale/2.0).round(),
          self.z
      )
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




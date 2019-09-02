use std::num::sqrt;

#[derive(Copy, Clone)]
struct Vec3 {
    e : [f32, 3],
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 { e: [0, 0, 0] }
    }

    pub fn from_scalar(t: f32) -> Vec3 {
        Vec3 { e: [t, t, t] }
    }

    pub fn from_values(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn from_other(other: Vec3) -> Vec3 {
        Vec3 { e: [other.x, other.y, other.z]}
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vec3, b: Vec3) {
        Vec3 { e: [a.y * b.z - a.z * b.y, 
                   a.z * b.x - a.x * b.z, 
                   a.x * b.y - a.y * b.x] 
        }
    }
}

impl Vec3 {
    pub fn x(&self) -> f32 { e[0] }
    pub fn y(&self) -> f32 { e[1] }
    pub fn z(&self) -> f32 { e[2] }

    pub fn set_x(&self, t: f32) { e[0] = t; }
    pub fn set_y(&self, t: f32) { e[1] = t; }
    pub fn set_z(&self, t: f32) { e[2] = t; }

    pub fn r(&self) -> f32 { e[0] }
    pub fn g(&self) -> f32 { e[1] }
    pub fn b(&self) -> f32 { e[2] }

    pub fn set_r(&self, t: f32) { e[0] = t; }
    pub fn set_g(&self, t: f32) { e[1] = t; }
    pub fn set_b(&self, t: f32) { e[2] = t; }

    pub fn set(&self, x: f32, y:f32, z:f32) {
        e[0] = x;
        e[1] = y;
        e[2] = z;
    }

    pub fn length(&self) -> f32 {
        (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt()
    }

    pub fn sqr_length(&self) -> f32 {
        e[0] * e[0] + e[1] * e[1] + e[2] * e[2]
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { e: [self.x + other.x, self.y + other.y, self.z + other.z]}
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f32) -> Vec3 {
        Vec3 { e: [self.x + other, self.y + other, self.z + other]}
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) {
        Vec3 { e: [self.x - other.x, self.y - other.y, self.z - other.z]}
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f32) {
        Vec3 { e: [self.x - other, self.y - other, self.z - other]}
    }
}
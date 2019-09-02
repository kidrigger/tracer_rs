use std::f32;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Copy, Clone)]
struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 {
            e: [0f32, 0f32, 0f32],
        }
    }

    pub fn from_scalar(t: f32) -> Vec3 {
        Vec3 { e: [t, t, t] }
    }

    pub fn from_values(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn from_other(other: Vec3) -> Vec3 {
        Vec3 {
            e: [other.x(), other.y(), other.z()],
        }
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.x() * b.x() + a.y() * b.y() + a.z() * b.z()
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            e: [
                a.y() * b.z() - a.z() * b.y(),
                a.z() * b.x() - a.x() * b.z(),
                a.x() * b.y() - a.y() * b.x(),
            ],
        }
    }
}

impl Vec3 {
    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn set_x(&mut self, t: f32) {
        self.e[0] = t;
    }
    pub fn set_y(&mut self, t: f32) {
        self.e[1] = t;
    }
    pub fn set_z(&mut self, t: f32) {
        self.e[2] = t;
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }
    pub fn g(&self) -> f32 {
        self.e[1]
    }
    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn set_r(&mut self, t: f32) {
        self.e[0] = t;
    }
    pub fn set_g(&mut self, t: f32) {
        self.e[1] = t;
    }
    pub fn set_b(&mut self, t: f32) {
        self.e[2] = t;
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.e[0] = x;
        self.e[1] = y;
        self.e[2] = z;
    }

    pub fn length(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn sqr_length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] + other, self.e[1] + other, self.e[2] + other],
        }
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [other.e[0] + self, other.e[1] + self, other.e[2] + self],
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] - other, self.e[1] - other, self.e[2] - other],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [other.e[0] * self, other.e[1] * self, other.e[2] * self],
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] / other.e[0],
                self.e[1] / other.e[1],
                self.e[2] / other.e[2],
            ],
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] / other, self.e[1] / other, self.e[2] / other],
        }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self / other.e[0], self / other.e[1], self / other.e[2]],
        }
    }
}

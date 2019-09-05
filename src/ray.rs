use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pos: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn origin(self) -> Vec3 {
        self.pos
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn normalize(mut self) {
        self.dir.normalize();
    }

    pub fn point_at(self, t: f32) -> Vec3 {
        t * self.dir.normalized() + self.pos
    }
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { pos: a, dir: b }
    }
}

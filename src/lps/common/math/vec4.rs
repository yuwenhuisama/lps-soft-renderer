use std::ops::{Mul, Add};

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const Zero: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.0);
    pub const One: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn do_multiply_scalar(&self, n: f32) -> Vec4 {
        Vec4::new(self.x * n, self.y * n, self.z * n, self.w * n)
    }

    pub fn do_add(&self, other: &Vec4) -> Vec4{
        Vec4::new(self.x + other.x , self.y + other.y, self.z + other.z, self.w + other.w)
    }

    pub fn do_dot(&self, other: &Vec4) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn lerp(v1: Vec4, v2: Vec4, factor: f32) -> Vec4 {
        (1.0 - factor) * v1 + factor * v2
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        self.do_multiply_scalar(rhs)
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        rhs.do_multiply_scalar(self)
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = f32;

    fn mul(self, rhs: Vec4) -> Self::Output {
        self.do_dot(&rhs)
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.do_add(&rhs)
    }
}

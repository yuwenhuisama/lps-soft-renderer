use std::ops::{Mul, Add};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const Zero: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    pub const One: Vec3 = Vec3::new(1.0, 1.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn do_multiply_scalar(&self, n: f32) -> Vec3 {
        Vec3::new(self.x * n, self.y * n, self.z * n)
    }

    pub fn do_add(&self, other: &Vec3) -> Vec3{
        Vec3::new(self.x + other.x , self.y + other.y, self.z + other.z)
    }

    pub fn do_dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn lerp(v1: Vec3, v2: Vec3, factor: f32) -> Vec3 {
        (1.0 - factor) * v1 + factor * v2
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        self.do_multiply_scalar(rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.do_multiply_scalar(self)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.do_dot(&rhs)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.do_add(&rhs)
    }
}

use std::ops::{Mul, Add};

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2::new(0.0, 0.0);
    pub const ONE: Vec2 = Vec2::new(1.0, 1.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn do_multiply_scalar(&self, n: f32) -> Vec2 {
        Vec2::new(self.x * n, self.y * n)
    }

    pub fn do_add(&self, other: &Vec2) -> Vec2{
        Vec2::new(self.x + other.x , self.y + other.y)
    }

    pub fn do_dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn lerp(v1: Vec2, v2: Vec2, factor: f32) -> Vec2 {
        (1.0 - factor) * v1 + factor * v2
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        self.do_multiply_scalar(rhs)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs.do_multiply_scalar(self)
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = f32;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self.do_dot(&rhs)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.do_add(&rhs)
    }
}

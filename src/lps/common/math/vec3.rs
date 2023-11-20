use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    pub const ONE: Vec3 = Vec3::new(1.0, 1.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn do_multiply_scalar(&self, n: f32) -> Vec3 {
        Vec3::new(self.x * n, self.y * n, self.z * n)
    }

    pub fn do_add(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn do_add_scalar(&self, n: f32) -> Vec3 {
        Vec3::new(self.x + n, self.y + n, self.z + n)
    }

    pub fn do_dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn lerp(v1: Vec3, v2: Vec3, factor: f32) -> Vec3 {
        (1.0 - factor) * v1 + factor * v2
    }

    pub fn len(&self) -> f32 {
        self.do_dot(self).sqrt()
    }

    pub fn normal(&self) -> Vec3 {
        let len = self.len();
        Vec3::new(self.x / len, self.y / len, self.z / len)
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

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.do_multiply_scalar(rhs);
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self.do_multiply_scalar(1.0 / rhs)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = self.do_multiply_scalar(1.0 / rhs);
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.do_add(&rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.do_add(&rhs);
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        self.do_add_scalar(rhs)
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        rhs.do_add_scalar(self)
    }
}

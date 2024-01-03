use crate::lps::common::math::vec3::Vec3;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

use super::vec4::Vec4;

#[derive(Clone, Copy, Debug)]
pub struct Mat4x4 {
    col_: usize, // w
    row_: usize, // h
    matrix: [f32; 4 * 4],
}

impl Mat4x4 {
    pub fn new_with_zero() -> Mat4x4 {
        Mat4x4 {
            row_: 4,
            col_: 4,
            matrix: [0.0; 4 * 4],
        }
    }

    pub fn new_with_value(value: f32) -> Mat4x4 {
        Mat4x4 {
            row_: 4,
            col_: 4,
            matrix: [value; 4 * 4],
        }
    }

    pub fn new_with_init(mat: [f32; 4 * 4]) -> Mat4x4 {
        Mat4x4 {
            row_: 4,
            col_: 4,
            matrix: mat,
        }
    }

    pub fn identity() -> Mat4x4 {
        let mut mat = Mat4x4::new_with_zero();
        for i in 0..4 {
            mat[i][i] = 1.0;
        }
        return mat;
    }

    pub fn rotate_x_mat(angle: f32) -> Mat4x4 {
        #[rustfmt::skip]
        return Mat4x4::new_with_init([
            1.0, 0.0, 0.0, 0.0,
            0.0, angle.cos(), -angle.sin(), 0.0,
            0.0, angle.sin(), angle.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]).trans();
    }

    pub fn rotate_y_mat(angle: f32) -> Mat4x4 {
        #[rustfmt::skip]
        return Mat4x4::new_with_init([
            angle.cos(), 0.0, angle.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            -angle.sin(), 0.0, angle.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]).trans();
    }

    pub fn rotate_z_mat(angle: f32) -> Mat4x4 {
        #[rustfmt::skip]
        return Mat4x4::new_with_init([
            angle.cos(), -angle.sin(), 0.0, 0.0,
            angle.sin(), angle.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]).trans();
    }

    pub fn rotate_axis_mat(angle: f32, axis: Vec3) -> Mat4x4 {
        let mut mat = Mat4x4::identity();
        let cos = angle.cos();
        let sin = angle.sin();
        let one_minus_cos = 1.0 - cos;
        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        mat[0][0] = cos + x * x * one_minus_cos;
        mat[0][1] = x * y * one_minus_cos - z * sin;
        mat[0][2] = x * z * one_minus_cos + y * sin;
        mat[1][0] = y * x * one_minus_cos + z * sin;
        mat[1][1] = cos + y * y * one_minus_cos;
        mat[1][2] = y * z * one_minus_cos - x * sin;
        mat[2][0] = z * x * one_minus_cos - y * sin;
        mat[2][1] = z * y * one_minus_cos + x * sin;
        mat[2][2] = cos + z * z * one_minus_cos;
        mat[3][3] = 1.0;

        return mat.trans();
    }

    pub fn viewport_mat(ox: i32, oy: i32, width: i32, height: i32) -> Mat4x4 {
        let mut mat = Mat4x4::identity();
        mat[0][0] = width as f32 / 2.0;
        mat[3][0] = ox as f32 + width as f32 / 2.0;
        mat[1][1] = height as f32 / 2.0;
        mat[3][1] = oy as f32 + height as f32 / 2.0;
        return mat;
    }

    // V = R*T
    // T = [  1 , 0 , 0 , -eyex          R = [  Right , 0
    //        0 , 1 , 0 , -eyey                   UP  , 0
    //        0 , 0 , 1 , -eyez               - Front , 0
    //        0 , 0 , 0 ,   1   ]                 0   , 1 ]
    //V =  [  Right  ,  - Right·eye
    //          UP   ,  - UP·eye
    //       -Front  ,   Front·eye
    //         0     ,       1        ]
    pub fn view_mat(pos: &Vec3, front: &Vec3, right: &Vec3, up: &Vec3) -> Mat4x4 {
        #[rustfmt::skip]
        return Mat4x4::new_with_init([
            right.x, right.y, right.z, -(*right * *pos),
            up.x, up.y, up.z, -(*up * *pos),
            -front.x, -front.y, -front.z, *front * *pos,
            0.0, 0.0, 0.0, 1.0,
        ]);
    }

    //M = [   1/aspect*tan(fov/2),       0      ,         0      ,       0
    //               0  ,         1/tan(fov/2)  ,         0      ,       0
    //               0  ,                0      ,  - (f+n)/(f-n) ,  -2fn/(f-n)
    //               0  ,                0      ,         -1     ,       0     ]
    pub fn perspective_mat(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4x4 {
        let mut mat = Mat4x4::new_with_zero();
        let tan_half_fov = (fov / 2.0).tan();
        mat[0][0] = 1.0 / (aspect * tan_half_fov);
        mat[1][1] = 1.0 / tan_half_fov;
        mat[2][2] = -(far + near) / (far - near);
        mat[2][3] = -1.0;
        mat[3][2] = -2.0 * far * near / (far - near);
        return mat;
    }

    pub fn width(&self) -> usize {
        self.col_
    }

    pub fn height(&self) -> usize {
        self.row_
    }

    pub fn row(&self) -> usize {
        self.row_
    }

    pub fn column(&self) -> usize {
        self.col_
    }

    pub fn set(&mut self, x: usize, y: usize, val: f32) {
        assert!(x < self.height() && y < self.width());
        let idx = x * self.width() + y;
        self.matrix[idx] = val;
    }

    pub fn at(&self, x: usize, y: usize) -> f32 {
        let idx = x * self.col_ + y;
        assert!(
            idx < self.matrix.len(),
            "x={}, y={}, col={}, row={}",
            x,
            y,
            self.col_,
            self.row_
        );
        self.matrix[idx]
    }

    pub fn trans(&self) -> Mat4x4 {
        let mut res = Mat4x4::new_with_zero();
        for i in 0..self.height() {
            for j in 0..self.width() {
                res.set(j, i, self.at(i, j));
            }
        }
        return res;
    }

    fn multiply_scalar(&self, n: f32) -> Mat4x4 {
        let mut res = Mat4x4::new_with_zero();

        let mut idx = 0;
        for ele in &self.matrix {
            idx += 1;
            res.matrix[idx] = *ele * n;
        }

        return res;
    }

    fn multiply_vec4(&self, vec: &Vec4) -> Vec4 {
        let rhs = vec![vec.x, vec.y, vec.z, vec.w];
        let mut res = vec![0.0, 0.0, 0.0, 0.0];

        for i in 0..self.height() {
            for j in 0..self.width() {
                res[i] += self.at(i, j) * rhs[j];
            }
        }

        return Vec4::new(res[0], res[1], res[2], res[3]);
    }

    fn to_neg(&self) -> Mat4x4 {
        let len = self.matrix.len();
        let mut mat = [0.0; 4 * 4];

        for i in 0..len {
            mat[i] = -self.matrix[i];
        }

        Mat4x4::new_with_init(mat)
    }

    fn add_mat(&self, other: &Mat4x4) -> Mat4x4 {
        let len = self.matrix.len();
        let mut mat = [0.0; 4 * 4];
        for i in 0..len {
            mat[i] = self.matrix[i] + other.matrix[i];
        }

        Mat4x4::new_with_init(mat)
    }

    fn add_mat_to_self(&mut self, other: &Mat4x4) {
        let len = self.matrix.len();
        for i in 0..len {
            self.matrix[i] += other.matrix[i];
        }
    }

    fn multiply_mat(&self, other: &Mat4x4) -> Mat4x4 {
        let mut res = Mat4x4::new_with_zero();

        for i in 0..self.height() {
            for j in 0..other.width() {
                let idx = i * other.width() + j;
                for k in 0..self.width() {
                    res.matrix[idx] += self.at(i, k) * other.at(k, j);
                }
            }
        }
        return res;
    }
}

impl Index<usize> for Mat4x4 {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[(index * self.col_)..((index + 1) * self.col_)]
    }
}

impl IndexMut<usize> for Mat4x4 {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [f32] {
        &mut self.matrix[(index * self.col_)..((index + 1) * self.col_)]
    }
}

impl Neg for Mat4x4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.to_neg()
    }
}

// impl Neg for &Mat4x4 {
//     type Output = Mat4x4;

//     fn neg(self) -> Self::Output {
//         self.get_neg()
//     }
// }

impl Mul<Self> for Mat4x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply_mat(&rhs)
    }
}

impl Mul<Vec4> for Mat4x4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        self.multiply_vec4(&rhs)
    }
}

// impl Mul<Self> for &Mat4x4 {
//     type Output = Mat4x4;

//     fn mul(self, rhs: Self) -> Self::Output {
//         self.multiply_mat(rhs)
//     }
// }

impl Add for Mat4x4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_mat(&rhs)
    }
}

// impl Add for &Mat4x4 {
//     type Output = Mat4x4;

//     fn add(self, rhs: Self) -> Self::Output {
//         self.add_mat(rhs)
//     }
// }

impl AddAssign<Self> for Mat4x4 {
    fn add_assign(&mut self, rhs: Self) {
        self.add_mat_to_self(&rhs)
    }
}

// impl AddAssign<&Self> for Mat4x4 {
//     fn add_assign(&mut self, rhs: &Self) {
//         self.add_mat_to_self(rhs)
//     }
// }

impl Sub for Mat4x4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

// impl Sub for &Mat4x4 {
//     type Output = Mat4x4;

//     fn sub(self, rhs: Self) -> Self::Output {
//         self + &-rhs
//     }
// }

impl SubAssign for Mat4x4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.add_mat_to_self(&-rhs);
    }
}

// impl SubAssign<&Self> for Mat4x4 {
//     fn sub_assign(&mut self, rhs: &Self) {
//         self.add_mat_to_self(&-rhs);
//     }
// }

impl MulAssign<Self> for Mat4x4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.multiply_mat(&rhs);
    }
}

// impl MulAssign<&Self> for Mat4x4 {
//     fn mul_assign(&mut self, rhs: &Self) {
//         *self = self.multiply_mat(&rhs);
//     }
// }

impl MulAssign<f32> for Mat4x4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.multiply_scalar(rhs);
    }
}

impl PartialEq for Mat4x4 {
    fn eq(&self, other: &Self) -> bool {
        self.row_ == other.row_ && self.col_ == other.col_ && self.matrix == other.matrix
    }
}

use std::ops::{Index, IndexMut, Mul, MulAssign, Add, Neg, Sub, AddAssign, SubAssign};

use super::vec4::Vec4;

#[derive(Clone, Copy, Debug)]
pub struct Mat4x4 {
    col_: usize, // w
    row_: usize,  // h
    matrix: [f32; 4 * 4]
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
        assert!(idx < self.matrix.len(), "x={}, y={}, col={}, row={}", x, y, self.col_, self.row_);
        self.matrix[idx]
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
        let mut res = Vec4::new(0.0, 0.0, 0.0, 0.0);

        for i in 0..self.height() {
            let idx = i * self.width();
            res.x += self.matrix[idx] * vec.x;
            res.y += self.matrix[idx + 1] * vec.y;
            res.z += self.matrix[idx + 2] * vec.z;
            res.w += self.matrix[idx + 3] * vec.w;
        }

        return res;
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

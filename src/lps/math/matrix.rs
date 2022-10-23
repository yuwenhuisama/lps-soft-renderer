use std::{ops::{Index, IndexMut, Mul, MulAssign, Add, Neg, Sub, AddAssign, SubAssign}, vec};

pub struct Matrix  {
    col_: usize,  // w
    row_: usize,  // h

    matrix_: Vec::<f32>,
}

impl Matrix {
    pub fn newWithZero(col: usize, row: usize) -> Matrix {
        assert!(row > 0 && col > 0);
        Matrix {
            row_: row,
            col_: col,
            matrix_: vec![0.0; row * col],
        }
    }

    pub fn newWithInit(col: usize, row: usize, mat:Vec::<f32>) -> Matrix {
        assert_eq!(row * col, mat.len());
        Matrix { 
            row_: row,
            col_: col,
            matrix_: mat
        }
    }

    pub fn width(&self) -> usize {
        self.col_
    }

    pub fn height(&self) -> usize {
        self.row_
    }

    pub fn set(&mut self, x: usize, y: usize, val: f32) {
        assert!(x < self.height() && y < self.width());
        let idx = x * self.width() + y;
        self.matrix_[idx] = val;
    }

    pub fn row(&self) -> usize {
        self.row_
    }

    pub fn column(&self) -> usize {
        self.col_
    }

    pub fn at(&self, x: usize, y: usize) -> f32 {
        let idx = x * self.col_ + y;
        assert!(idx < self.matrix_.len(), "x={}, y={}, col={}, row={}", x, y, self.col_, self.row_);
        self.matrix_[idx]
    }

    fn multiply_scalar(&self, n: f32) -> Matrix {
        let mut res = Matrix::newWithZero(self.column(), self.row());

        let mut idx = 0;
        for ele in &self.matrix_ {
            idx += 1;
            res.matrix_[idx] = *ele * n;
        }

        return res;
    }

    fn get_neg(&self) -> Matrix {
        let len = self.matrix_.len();
        let mut mat = vec![0.0; len];

        for i in 0..len {
            mat[i] = -self.matrix_[i];
        }

        Matrix::newWithInit(self.column(), self.column(), mat)
    }

    fn add_mat(&self, other: &Matrix) -> Matrix {
        assert!(self.width() == other.width() && self.height() == other.height());

        let len = self.matrix_.len();
        let mut mat = vec![0.0; len];
        for i in 0..len {
            mat[i] = self.matrix_[i] + other.matrix_[i];
        }

        Matrix::newWithInit(self.column(), self.column(), mat)
    }

    fn add_mat_to_self(&mut self, other: &Matrix) {
        assert!(self.width() == other.width() && self.height() == other.height());

        let len = self.matrix_.len();
        for i in 0..len {
            self.matrix_[i] += other.matrix_[i];
        }
    }

    fn multiply_mat(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.width(), other.height());

        let mut res = Matrix::newWithZero(self.height(), other.width());

        for i in 0..self.height() {
            for j in 0..other.width() {
                let idx = i * other.width() + j;
                for k in 0..self.width() {
                    res.matrix_[idx] += self.at(i, k) * other.at(k, j);
                }
            }
        }
        return res;
    }
}

impl Index<usize> for Matrix  {
    type Output = [f32];

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.matrix_[(index * self.col_) .. self.row_]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [f32] {
        &mut self.matrix_[(index * self.col_) .. self.row_]
    }
}

impl Neg for Matrix {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.get_neg()
    }
}

impl Neg for &Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        self.get_neg()
    }
}

impl Mul<Self> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply_mat(&rhs)
    }
}

impl Mul<Self> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply_mat(rhs)
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_mat(&rhs)
    }
}

impl Add for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_mat(rhs)
    }
}

impl AddAssign<Self> for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        self.add_mat_to_self(&rhs)
    }
}

impl AddAssign<&Self> for Matrix {
    fn add_assign(&mut self, rhs: &Self) {
        self.add_mat_to_self(rhs)
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Self::Output {
        self + &-rhs
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Self) {
        self.add_mat_to_self(&-rhs);
    }
}

impl SubAssign<&Self> for Matrix {
    fn sub_assign(&mut self, rhs: &Self) {
        self.add_mat_to_self(&-rhs);
    }
}

impl MulAssign<Self> for Matrix {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.multiply_mat(&rhs);
    }
}

impl MulAssign<&Self> for Matrix {
    fn mul_assign(&mut self, rhs: &Self) {
        *self = self.multiply_mat(&rhs);
    }
}

impl MulAssign<f32> for Matrix {
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.multiply_scalar(rhs);
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.row_ == other.row_ && self.col_ == other.col_ && self.matrix_ == other.matrix_
    }
}

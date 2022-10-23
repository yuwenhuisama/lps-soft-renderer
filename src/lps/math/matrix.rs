use std::ops::{Index, IndexMut};

pub struct Matrix  {
    row_: usize,  // h
    col_: usize,  // w

    matrix_: Vec::<f32>,
}

impl Matrix {
    pub fn new(row: usize, col: usize) -> Matrix{
        Matrix {
            row_: row,
            col_: col,
            matrix_: vec![0.0; row * col],
        }
    }

    pub fn row(&self) -> usize {
        self.row_
    }

    pub fn col(&self) -> usize {
        self.col_
    }

    pub fn at(&self, x: usize, y: usize) -> f32 {
        self.matrix_[x * self.col_ + y]
    }

    pub fn dot(&self, n: f32) -> Matrix {
        let mut res = Matrix::new(self.row_, self.col_);

        let mut idx = 0;
        for ele in &self.matrix_ {
            idx += 1;
            res.matrix_[idx] = *ele * n;
        }

        return res;
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        let mut res = Matrix::new(self.row_, other.col_);

        let x = self.row_;
        let y = other.col_;
        let z = self.col_;

        // r * c
        for i in 0..x {
            for j in 0..z {
                let idx = i * other.col() + j;
                res.matrix_[idx] = 0.0;
                for k in 0..y {
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
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix_[(index * self.col_) .. self.row_]
    }
}
impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.row_ == other.row_ && self.col_ == other.col_ && self.matrix_ == other.matrix_
    }
}

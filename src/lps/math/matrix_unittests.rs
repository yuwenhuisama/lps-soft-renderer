#[cfg(test)]
mod tests {
    use crate::lps::math::matrix::Matrix;

    #[test]
    fn test_matrix_index() {
        let mat1 = Matrix::newWithInit(3, 4, vec![
            1.0,  2.0,  3.0,
            4.0,  5.0,  6.0,
            7.0,  8.0,  9.0,
            10.0, 11.0, 12.0,]);
        assert_eq!(mat1.at(2, 1), 8.0);
        assert_eq!(mat1.at(1, 2), 6.0);
    }

    #[test]
    fn test_matrix_multiply() {
        let mut mat1 = Matrix::newWithInit(3, 4, vec![
            1.0,  2.0,  3.0,
            4.0,  5.0,  6.0,
            7.0,  8.0,  9.0,
            10.0, 11.0, 12.0,]);
        let mat2 = Matrix::newWithInit(4, 3, vec![
            3.0,  4.0,  5.0,  6.0, 
            7.0,  8.0,  9.0,  10.0,
            11.0, 12.0, 13.0, 14.0,
         ]);

        let mat3 = mat1 * mat2;
        let res = Matrix::newWithInit(4, 4, vec![
            50.0,  56.0,  62.0,  68.0,
            113.0, 128.0, 143.0, 158.0,
            176.0, 200.0, 224.0, 248.0,
            239.0, 272.0, 305.0, 338.0,
        ]);

        assert!(mat3 == res);
    }
}
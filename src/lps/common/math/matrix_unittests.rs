#[cfg(test)]
mod tests {
    use crate::lps::common::math::mat4x4::Mat4x4;
    use crate::lps::common::math::vec4::Vec4;

    #[test]
    fn test_matrix_index() {
        #[rustfmt::skip]
        let mat1 = Mat4x4::new_with_init([
            1.0,  2.0,  3.0, 4.0,
            4.0,  5.0,  6.0, 7.0,
            7.0,  8.0,  9.0, 10.0,
            10.0, 11.0, 12.0,13.0,]);
        assert_eq!(mat1.at(2, 1), 8.0);
        assert_eq!(mat1.at(1, 2), 6.0);
    }

    #[test]
    fn test_matrix_multiply() {
        #[rustfmt::skip]
        let mat1 = Mat4x4::new_with_init([
            1.0,  2.0,  3.0, 4.0,
            4.0,  5.0,  6.0, 7.0,
            7.0,  8.0,  9.0, 10.0,
            10.0, 11.0, 12.0, 13.0]);

        #[rustfmt::skip]
        let mat2 = Mat4x4::new_with_init([
            3.0,  4.0,  5.0,  6.0, 
            7.0,  8.0,  9.0,  10.0,
            11.0, 12.0, 13.0, 14.0,
            15.0, 16.0, 17.0, 18.0
         ]);

        let mat3 = mat1 * mat2;

        #[rustfmt::skip]
        let res = Mat4x4::new_with_init([
            110.0, 120.0, 130.0, 140.0,
            218.0, 240.0, 262.0, 284.0,
            326.0, 360.0, 394.0, 428.0,
            434.0, 480.0, 526.0, 572.0,         
        ]);

        assert!(mat3 == res);
    }

    #[test]
    fn test_matrix_multiply_vec4() {
        #[rustfmt::skip]
            let mat1 = Mat4x4::new_with_init([
            1.0,  2.0,  3.0, 4.0,
            4.0,  5.0,  6.0, 7.0,
            7.0,  8.0,  9.0, 10.0,
            10.0, 11.0, 12.0, 13.0]);

        let vec4 = Vec4::new(1.0, 2.0, 3.0, 4.0);

        let res = mat1 * vec4;

        let right = Vec4::new(30.0, 60.0, 90.0, 120.0);

        assert!(res == right);
    }
}

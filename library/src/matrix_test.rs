use demonstrate::demonstrate;

demonstrate! {
    describe "Matrix4" {
        use crate::matrix::Matrix4;
        use crate::tuple::Tuple;

        it "should construct a matrix (order: 4) from values, and access them" {
            let matrix = Matrix4::new(&[
                1.0, 2.0, 3.0, 4.0,
                5.5, 6.5, 7.5, 8.5,
                9.0, 10.0, 11.0, 12.0,
                13.5, 14.5, 15.5, 16.5
            ]);

            assert_float_absolute_eq!(matrix[0][0], 1.0);
            assert_float_absolute_eq!(matrix[0][3], 4.0);
            assert_float_absolute_eq!(matrix[1][0], 5.5);
            assert_float_absolute_eq!(matrix[1][2], 7.5);
            assert_float_absolute_eq!(matrix[2][2], 11.0);
            assert_float_absolute_eq!(matrix[3][0], 13.5);
            assert_float_absolute_eq!(matrix[3][2], 15.5);
        }

        describe "comparison" {
            it "should return true for equal matrices" {
                let matrix1 = Matrix4::new(&[
                    1.0, 2.0, 3.0, 4.0,
                    5.5, 6.5, 7.5, 8.5,
                    9.0, 10.0, 11.0, 12.0,
                    13.5, 14.5, 15.5, 16.5
                ]);

                let matrix2 = Matrix4::new(&[
                    1.0, 2.0, 3.0, 4.0,
                    5.5, 6.5, 7.5, 8.5,
                    9.0, 10.0, 11.0, 12.0,
                    13.5, 14.5, 15.5, 16.5
                ]);

                assert_eq!(matrix1, matrix2);
            }

            it "should return false for different matrices" {
                let matrix1 = Matrix4::new(&[
                    1.0, 2.0, 3.0, 4.0,
                    5.5, 6.5, 7.5, 8.5,
                    9.0, 10.0, 11.0, 12.0,
                    13.5, 14.5, 15.5, 16.5
                ]);

                let matrix2 = Matrix4::new(&[
                    1.0, 2.0, 3.0, 4.0,
                    5.5, 6.5, 7.5, 8.5,
                    9.0, 10.0, 11.0, 12.0,
                    13.5, 14.5, 15.5, 16.4
                ]);

                assert!(!(matrix1 == matrix2));
            }
        } // describe "comparison"

        describe "multiplication" {
            it "should be performed against another matrix" {
                let matrix1 = Matrix4::new(&[
                    1, 2, 3, 4,
                    5, 6, 7, 8,
                    9, 8, 7, 6,
                    5, 4, 3, 2,
                ]);

                let matrix2 = Matrix4::new(&[
                    -2, 1, 2,  3,
                     3, 2, 1, -1,
                     4, 3, 6,  5,
                     1, 2, 7,  8,
                ]);

                let expected_result = Matrix4::new(&[
                    20, 22,  50,  48,
                    44, 54, 114, 108,
                    40, 58, 110, 102,
                    16, 26,  46,  42,
                ]);

                assert_eq!(matrix1 * matrix2, expected_result);
            }

            it "should be performed against a tuple" {
                let matrix = Matrix4::new(&[
                    1, 2, 3, 4,
                    2, 4, 4, 2,
                    8, 6, 4, 1,
                    0, 0, 0, 1,
                ]);

                let tuple = Tuple::new(
                    1,
                    2,
                    3,
                    1,
                );

                let expected_result = Tuple::new(
                    18,
                    24,
                    33,
                    1,
                );

                assert_eq!(matrix * tuple, expected_result);
            }

            it "against an identity matrix should return a matrix equal to the first one" {
                let matrix = Matrix4::new(&[
                    1, 2, 3, 4,
                    2, 4, 4, 2,
                    8, 6, 4, 1,
                    0, 0, 0, 1,
                ]);

                let expected_result = matrix.clone();

                assert_eq!(matrix * Matrix4::identity(), expected_result)
            }
        } // describe "multiplication"
    }

    describe "Matrix2" {
        use crate::matrix::Matrix2;

        it "should construct a matrix (order: 2) from values, and access them" {
            let matrix = Matrix2::new(&[
                -3.0, 5.0,
                1.0, -2.0,
            ]);

            assert_float_absolute_eq!(matrix[0][0], -3.0);
            assert_float_absolute_eq!(matrix[0][1], 5.0);
            assert_float_absolute_eq!(matrix[1][0], 1.0);
            assert_float_absolute_eq!(matrix[1][1], -2.0);
        }
    }

    describe "Matrix3" {
        use crate::matrix::Matrix3;

        it "should construct a matrix (order: 3) from values, and access them" {
            let matrix = Matrix3::new(&[
                -3.0,  5.0,  0.0,
                 1.0, -2.0, -7.0,
                 0.0,  1.0,  1.0,
            ]);

            assert_float_absolute_eq!(matrix[0][0], -3.0);
            assert_float_absolute_eq!(matrix[1][1], -2.0);
            assert_float_absolute_eq!(matrix[2][2], 1.0);
        }
    }
}

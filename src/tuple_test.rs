use demonstrate::demonstrate;

use crate::tuple::{Tuple, EPSILON, POINT_TYPE, VECTOR_TYPE};

demonstrate! {
    describe "CPU" {
        use super::*;

        context "Tuple" {
            context "with w=1_0" {
                it "is a point" {
                    let tuple = Tuple(2.0, 4.0, 8.0, 1.0);

                    match tuple {
                        Tuple(_, _, _, type_value) => {
                          assert_float_absolute_eq!(type_value, POINT_TYPE);
                        }
                    }
                }
            }

            context "with w=0_0" {
                it "is a vector" {
                    let tuple = Tuple(2.0, 4.0, 8.0, 0.0);

                    match tuple {
                        Tuple(_, _, _, type_value) => {
                          assert_float_absolute_eq!(type_value, VECTOR_TYPE);
                        }
                    }
                }
            }

            context "::point" {
                it "creates a tuple with w=1_0" {
                    let tuple = Tuple::point(2.0, 4.0, 8.0);

                    match tuple {
                        Tuple(_, _, _, type_value) => {
                          assert_float_absolute_eq!(type_value, POINT_TYPE);
                        }
                    }
                }
            }

            context "::vector" {
                it "creates a tuple with w=0_0" {
                    let tuple = Tuple::vector(2.0, 4.0, 8.0);

                    match tuple {
                        Tuple(_, _, _, type_value) => {
                          assert_float_absolute_eq!(type_value, VECTOR_TYPE);
                        }
                    }
                }
            }

            // For simplicity, ignore NaN.
            //
            it "equals other tuples with the same values, within epsilon" {
                let tuple1 = Tuple(1.0, 2.0, 3.0, 1.0);
                let tuple2 = Tuple(1.00000000001, 2.00000000001, 3.00000000001, 1.00000000001);

                assert_eq!(tuple1, tuple2);
            }

            it "can be added to another tuple" {
                let tuple1 = Tuple(3.0, -2.0, 5.0, 1.0);
                let tuple2 = Tuple(-2.0, 3.0, 1.0, 0.0);

                let expected_tuple = Tuple(1.0, 1.0, 6.0, 1.0);

                assert_eq!(tuple1 + tuple2, expected_tuple);
            }

            context "as point" {
                it "can be subtracted from a point" {
                    let tuple1 = Tuple::point(3.0, 2.0, 1.0);
                    let tuple2 = Tuple::point(5.0, 6.0, 7.0);

                    let expected_tuple = Tuple::vector(-2.0, -4.0, -6.0);

                    assert_eq!(tuple1 - tuple2, expected_tuple);
                }
            }

            context "as vector" {
                it "can be subtracted from a point" {
                    let tuple1 = Tuple::point(3.0, 2.0, 1.0);
                    let tuple2 = Tuple::vector(5.0, 6.0, 7.0);

                    let expected_tuple = Tuple::point(-2.0, -4.0, -6.0);

                    assert_eq!(tuple1 - tuple2, expected_tuple);
                }

                it "can be subtracted from a vector" {
                    let tuple1 = Tuple::vector(3.0, 2.0, 1.0);
                    let tuple2 = Tuple::vector(5.0, 6.0, 7.0);

                    let expected_tuple = Tuple::vector(-2.0, -4.0, -6.0);

                    assert_eq!(tuple1 - tuple2, expected_tuple);
                }
            }

            it "can be subtracted from the zero vector" {
                let tuple1 = Tuple::vector(0.0, 0.0, 0.0);
                let tuple2 = Tuple::vector(5.0, 6.0, 7.0);

                let expected_tuple = Tuple::vector(-5.0, -6.0, -7.0);

                assert_eq!(tuple1 - tuple2, expected_tuple);
            }

            // At this stage of the book, it's unclear why the book negates a non-meaningful tuple.
            //
            it "can be negated" {
                let tuple = Tuple(1.0, -2.0, 0.0, 2.0);

                let expected_tuple = Tuple(-1.0, 2.0, -0.0, -2.0);

                assert_eq!(-tuple, expected_tuple);
            }

            it "can be multiplied by a floating point factor" {
                let tuple = Tuple(1.0, -2.0, 0.0, 2.0);

                let expected_tuple = Tuple(2.5, -5.0, 0.0, 5.0);

                assert_eq!(tuple * 2.5, expected_tuple);
            }

            it "can be divided by a floating point factor" {
                let tuple = Tuple(1.0, -2.0, 0.0, 2.0);

                let expected_tuple = Tuple(2.0, -4.0, 0.0, 4.0);

                assert_eq!(tuple / 0.5, expected_tuple);
            }

            context "should have a magnitude" {
                it "as vector (-1, -2, -3)" {
                    let vector = Tuple::vector(-1.0, -2.0, -3.0);

                    let expected_magnitude = 14.0_f64.sqrt();

                    assert!(vector.magnitude() - expected_magnitude < EPSILON);
                }
            }
        }
    }
}

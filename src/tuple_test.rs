use demonstrate::demonstrate;

use crate::tuple::{Tuple, POINT_TYPE, VECTOR_TYPE};

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
        }
    }
}

use crate::has_float64_value::HasFloat64Value;
use crate::tuple::Tuple;

use crate::EPSILON;

use std::mem::MaybeUninit; // For the lulz
use std::ops::{Index, IndexMut, Mul};

pub trait Matrix:
    Index<usize> + IndexMut<usize> + PartialEq + Sized + Mul<Self> + Mul<Tuple>
{
    fn transpose(&self) -> Self;
}

// For more lulz.
//
macro_rules! matrix {
    ($name:ident, $order: literal) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            pub values: [[f64; $order]; $order],
        }

        impl $name {
            pub fn new<T: Copy + HasFloat64Value>(source_values: &[T]) -> Self {
                if source_values.len() != $order * $order {
                    panic!("Inappropriate number of source values");
                }

                let mut values: [[f64; $order]; $order] =
                    unsafe { MaybeUninit::uninit().assume_init() };

                for (row, source_row) in values.iter_mut().zip(source_values.chunks_exact($order)) {
                    for (value, source_value) in row.iter_mut().zip(source_row.iter()) {
                        *value = source_value.as_f64();
                    }
                }

                Self { values }
            }

            pub fn identity() -> Self {
                // Tee-hee-hee!
                //
                let mut source_values = [0; $order * $order];

                for i in 0..$order {
                    source_values[($order + 1) * i] = 1;
                }

                Self::new(&source_values)
            }
        }

        impl Matrix for $name {
            // Shame - the destructive version is very amusing, using the Rust swap API.
            //
            fn transpose(&self) -> Self {
                let mut values: [[f64; $order]; $order] =
                    unsafe { MaybeUninit::uninit().assume_init() };

                for y in 0..$order {
                    for x in 0..$order {
                        values[y][x] = self[x][y];
                    }
                }

                Self { values }
            }
        }

        impl Index<usize> for $name {
            type Output = [f64; $order];

            fn index(&self, y: usize) -> &Self::Output {
                &self.values[y]
            }
        }

        impl IndexMut<usize> for $name {
            fn index_mut(&mut self, y: usize) -> &mut [f64; $order] {
                &mut self.values[y]
            }
        }

        // Due to the epsilon handling, we can't use a direct/bitwise comparison.
        //
        impl PartialEq for $name {
            fn eq(&self, rhs: &Self) -> bool {
                self.values
                    .iter()
                    .zip(rhs.values.iter())
                    .all(|(row, rhs_row)| {
                        row.iter()
                            .zip(rhs_row.iter())
                            .all(|(value, rhs_value)| (value - rhs_value).abs() < EPSILON)
                    })
            }
        }

        impl Mul<$name> for $name {
            type Output = $name;

            fn mul(self, rhs: $name) -> Self::Output {
                let mut result: [[f64; $order]; $order] =
                    unsafe { MaybeUninit::uninit().assume_init() };

                for y in 0..$order {
                    for x in 0..$order {
                        result[y][x] = (0..$order).map(|k| self[y][k] * rhs[k][x]).sum();
                    }
                }

                $name { values: result }
            }
        }

        impl Mul<Tuple> for $name {
            type Output = Tuple;

            fn mul(self, rhs: Tuple) -> Self::Output {
                if $order != 4 {
                    panic!("Only matrices of order 4 are allowed to be multiplied by a Tuple");
                }

                let mut result = Tuple::uninitialized();

                for y in 0..$order {
                    result[y] = (0..$order).map(|k| self[y][k] * rhs[k]).sum();
                }

                result
            }
        }
    };
}

matrix!(Matrix2, 2);
matrix!(Matrix3, 3);
matrix!(Matrix4, 4);

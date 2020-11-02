use crate::EPSILON;

use std::mem::MaybeUninit; // For the lulz
use std::ops::{Index, IndexMut};

// For more lulz.
//
macro_rules! matrix {
    ($name:ident, $order: literal) => {
        #[derive(Debug)]
        pub struct $name {
            pub values: [[f64; $order]; $order],
        }

        impl $name {
            pub fn new(source_values: &[f64]) -> Self {
                if source_values.len() != $order * $order {
                    panic!("Inappropriate number of source values");
                }

                let mut values: [[f64; $order]; $order] =
                    unsafe { MaybeUninit::uninit().assume_init() };

                for (row, source_row) in values.iter_mut().zip(source_values.chunks_exact($order)) {
                    row.copy_from_slice(source_row);
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
    };
}

matrix!(Matrix2, 2);
matrix!(Matrix3, 3);
matrix!(Matrix4, 4);

use std::convert::TryInto;
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

                let mut values: [MaybeUninit<[f64; $order]>; $order] =
                    unsafe { MaybeUninit::uninit().assume_init() };

                for (row, source_row) in values.iter_mut().zip(source_values.chunks_exact($order)) {
                    *row = MaybeUninit::new(source_row.try_into().unwrap());
                }

                let values = unsafe { std::mem::transmute::<_, [[f64; $order]; $order]>(values) };

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
    };
}

matrix!(Matrix2, 2);
matrix!(Matrix3, 3);
matrix!(Matrix4, 4);

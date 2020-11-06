use crate::has_float64_value::HasFloat64Value;
use crate::tuple::Tuple;

use crate::EPSILON;

use std::{
    mem::{transmute, MaybeUninit},
    ops::{Index, IndexMut, Mul},
};

#[derive(Clone, Debug)]
pub struct Matrix {
    pub values: Vec<Vec<f64>>,
}

impl Matrix {
    // There isn't an entirely clean structure for the values to pass:
    //
    // - an array of arrays doesn't work, since the size must be known at compile time;
    // - a flat list could work with by appending an empty comment to each line, but as soon as a method
    //   is invoked on an entry (e.g. as_f64()), it alignes vertically.
    // - using slices works, although it's quite ugly.
    //
    // So, screw rustfmt, and just use `#[rustfmt::skip]`.
    //
    pub fn new<T: Copy + HasFloat64Value>(source_values: &[T]) -> Self {
        let order = (source_values.len() as f64).sqrt() as usize;

        if source_values.len() != order.pow(2) {
            panic!("Number of source values is not a square value");
        }

        let mut values = Vec::with_capacity(order);

        for source_row in source_values.chunks_exact(order) {
            values.push(
                source_row
                    .iter()
                    .map(|value| value.as_f64())
                    .collect::<Vec<_>>(),
            );
        }

        Self { values }
    }

    // For the lulz.
    //
    pub fn uninitialized(order: usize) -> Self {
        let values = (0..order)
            // Tee-hee-hee
            //
            .map(|_| vec![unsafe { MaybeUninit::<f64>::uninit().assume_init() }; order])
            .collect::<Vec<_>>();

        Self { values }
    }

    pub fn identity(order: usize) -> Self {
        let mut source_values = vec![0; order.pow(2)];

        for i in 0..order {
            source_values[(order + 1) * i] = 1;
        }

        Self::new(&source_values)
    }

    pub fn transpose(&self) -> Self {
        let mut result = self.values.clone();
        let order = self.values.len();

        for y in 0..order {
            for x in 0..order {
                result[y][x] = self[x][y];
            }
        }

        Self { values: result }
    }

    pub fn determinant(&self) -> f64 {
        if self.values.len() == 2 {
            self[0][0] * self[1][1] - self[0][1] * self[1][0]
        } else {
            self[0]
                .iter()
                .enumerate()
                .map(|(x, value)| value * self.cofactor(0, x))
                .sum()
        }
    }

    pub fn submatrix(&self, y: usize, x: usize) -> Matrix {
        let order = self.values.len();

        let mut result = Vec::with_capacity(order - 1);

        for current_y in 0..order {
            if current_y != y {
                let mut result_row = Vec::with_capacity(order - 1);

                for current_x in 0..order {
                    if current_x != x {
                        result_row.push(self[current_y][current_x]);
                    }
                }

                result.push(result_row);
            }
        }

        Self { values: result }
    }

    pub fn minor(&self, y: usize, x: usize) -> f64 {
        self.submatrix(y, x).determinant()
    }

    // Mad lulz here. Note that for portability, the bit shift should change depending on the arch.
    //
    pub fn cofactor(&self, y: usize, x: usize) -> f64 {
        let minor = self.minor(y, x);

        // The data type is irrelevant here, as long as it supports bit shifts (float doesn't).
        // usize is used for convenience on the next operation.
        //
        let minor_bits = unsafe { transmute::<_, usize>(minor) };

        // This is (0 for even/1 for odd), shifted to be the leftmost bit, so that it's in the sign position
        // of f64 values.
        //
        let sign_bits = (x + y) << 63;

        // Xor keeps the <destination sign> if the <sign operand> is 0, and changes it, if the <sign operand> is 1.
        //
        unsafe { transmute::<_, f64>(minor_bits ^ sign_bits) }
    }

    pub fn inverse(&self) -> Option<Matrix> {
        let determinant = self.determinant();

        if determinant == 0.0 {
            None
        } else {
            let order = self.values.len();

            let result = (0..order)
                .map(|y| {
                    (0..order)
                        // WATCH OUT! row/col inversion here.
                        //
                        .map(|x| self.cofactor(x, y) / determinant)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            Some(Self { values: result })
        }
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, y: usize) -> &Self::Output {
        &self.values[y]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, y: usize) -> &mut Vec<f64> {
        &mut self.values[y]
    }
}

// Due to the epsilon handling, we can't use a direct/bitwise comparison.
//
impl PartialEq for Matrix {
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

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let order = self.values.len();

        // Pre-initializing makes the multiplication logic easier to understand.
        //
        let mut result = Matrix::uninitialized(order);

        for y in 0..order {
            for x in 0..order {
                result[y][x] = (0..order).map(|k| self[y][k] * rhs[k][x]).sum();
            }
        }

        result
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let order = self.values.len();

        if order != 4 {
            panic!("Only matrices of order 4 are allowed to be multiplied by a Tuple");
        }

        let mut result = Tuple::uninitialized();

        for y in 0..order {
            result[y] = (0..order).map(|k| self[y][k] * rhs[k]).sum();
        }

        result
    }
}

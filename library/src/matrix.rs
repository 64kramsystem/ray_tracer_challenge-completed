use crate::has_float64_value::HasFloat64Value;
use crate::tuple::Tuple;

use crate::EPSILON;

use std::{
    mem::MaybeUninit,
    ops::{Index, IndexMut, Mul},
};

#[derive(Clone, Debug)]
pub struct Matrix {
    pub values: Vec<Vec<f64>>,
}

impl Matrix {
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
        if self.values.len() != 2 {
            panic!()
        }

        self[0][0] * self[1][1] - self[0][1] * self[1][0]
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

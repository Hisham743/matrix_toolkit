use crate::{Matrix, MatrixError};
use std::ops::{Add, Mul, Neg, Sub};

impl Add for &Matrix {
    type Output = Result<Matrix, MatrixError>;

    fn add(self, rhs: Self) -> Self::Output {
        self.element_wise_operation(rhs, |a, b| a + b)
    }
}

impl Sub for &Matrix {
    type Output = Result<Matrix, MatrixError>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.element_wise_operation(rhs, |a, b| a - b)
    }
}

impl Mul for &Matrix {
    type Output = Result<Matrix, MatrixError>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.columns != rhs.rows {
            return Err(MatrixError::DimensionMismatch);
        }

        let mut result_matrix = Matrix::new_zero_matrix(self.rows, rhs.columns).unwrap();
        for i in 0..result_matrix.rows {
            for j in 0..result_matrix.columns {
                let mut sum = 0.0;
                for k in 0..self.columns {
                    sum += self.data[i][k] * rhs.data[k][j]
                }
                result_matrix.data[i][j] = crate::round_to_five(sum);
            }
        }

        Ok(result_matrix)
    }
}

impl Neg for &Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

impl Mul<&Matrix> for f64 {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        Matrix::new_with_data(
            rhs.data
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|num| crate::round_to_five(self * num))
                        .collect()
                })
                .collect(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;

    #[test]
    fn add() {
        let [matrix_2x3, another_matrix_2x3, matrix_4x2] = test_utils::generic_examples();

        assert_eq!(
            Matrix::new_with_data(vec![vec![8.7, 22.7, 8.3], vec![16.0, 14.0, 11.2]]).unwrap(),
            (&matrix_2x3 + &another_matrix_2x3).unwrap()
        );
        assert_eq!(
            MatrixError::DimensionMismatch,
            (&matrix_2x3 + &matrix_4x2).unwrap_err()
        );
    }

    #[test]
    fn subtract() {
        let [matrix_2x3, another_matrix_2x3, matrix_4x2] = test_utils::generic_examples();

        assert_eq!(
            Matrix::new_with_data(vec![vec![5.7, 4.9, 1.9], vec![2.6, -8.6, 1.6]]).unwrap(),
            (&matrix_2x3 - &another_matrix_2x3).unwrap()
        );
        assert_eq!(
            MatrixError::DimensionMismatch,
            (&matrix_2x3 - &matrix_4x2).unwrap_err()
        );
    }

    #[test]
    fn scale() {
        let matrix_2x3 = &test_utils::generic_examples()[0];

        assert_eq!(
            Matrix::new_with_data(vec![vec![14.4, 27.6, 10.2], vec![18.6, 5.4, 12.8]]).unwrap(),
            2.0 * matrix_2x3
        );

        assert_eq!(
            Matrix::new_with_data(vec![vec![3.6, 6.9, 2.55], vec![4.65, 1.35, 3.2]]).unwrap(),
            0.5 * matrix_2x3
        );
    }

    #[test]
    fn multiplication() {
        let [matrix_2x3, _, matrix_4x2] = test_utils::generic_examples();

        assert_eq!(
            Matrix::new_with_data(vec![
                vec![131.46, 103.74, 91.28],
                vec![89.7, 60.0, 62.15],
                vec![109.47, 162.93, 76.96],
                vec![126.27, 110.43, 87.81],
            ])
            .unwrap(),
            (&matrix_4x2 * &matrix_2x3).unwrap()
        );

        assert_eq!(
            MatrixError::DimensionMismatch,
            (&matrix_2x3 * &matrix_4x2).unwrap_err()
        );
    }

    #[test]
    fn negative() {
        assert_eq!(
            Matrix::new_with_data(vec![vec![-7.2, -13.8, -5.1], vec![-9.3, -2.7, -6.4]]).unwrap(),
            -&test_utils::generic_examples()[0]
        )
    }
}

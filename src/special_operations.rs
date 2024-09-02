use crate::{Matrix, MatrixError};

impl Matrix {
    pub fn transpose(&self) -> Self {
        let mut transpose = Self::new_zero_matrix(self.columns, self.rows).unwrap();

        transpose
            .data
            .iter_mut()
            .enumerate()
            .for_each(|(row_index, row)| {
                row.iter_mut()
                    .enumerate()
                    .for_each(|(column_index, num)| *num = self.data[column_index][row_index])
            });

        transpose
    }

    pub fn trace(&self) -> Result<f64, MatrixError> {
        if !self.is_square() {
            return Err(MatrixError::NonSquareMatrix);
        }

        let mut trace = 0.0;
        for i in 0..self.rows {
            trace += self.data[i][i];
        }

        Ok(crate::round_to_five(trace))
    }

    pub fn determinant(&self) -> Result<f64, MatrixError> {
        if !self.is_square() {
            return Err(MatrixError::NonSquareMatrix);
        }

        Ok(match self.rows {
            1 => self.data[0][0],
            2 => crate::round_to_five(
                self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0],
            ),
            _ => {
                let mut upper_trig_matrix = self.clone();
                let n = upper_trig_matrix.rows;
                let mut row_swap_count = 0;

                for column in 0..n {
                    let mut pivot = upper_trig_matrix.data[column][column];
                    let mut pivot_row: usize = column;
                    for row in column..n {
                        let element = upper_trig_matrix.data[row][column];
                        if element.abs() > pivot.abs() {
                            pivot = element;
                            pivot_row = row;
                        }
                    }

                    if pivot_row != column {
                        upper_trig_matrix.data.swap(column, pivot_row);
                        row_swap_count += 1;
                    }

                    for row in (column + 1)..n {
                        let element_to_be_0 = upper_trig_matrix.data[row][column];
                        if let 0.0 = element_to_be_0 {
                            continue;
                        }

                        let factor = element_to_be_0 / upper_trig_matrix.data[column][column];
                        for i in 0..n {
                            upper_trig_matrix.data[row][i] -=
                                factor * upper_trig_matrix.data[column][i];
                        }
                    }
                }

                let mut result = 1.0;
                upper_trig_matrix
                    .data
                    .iter()
                    .enumerate()
                    .for_each(|(i, row)| result *= row[i]);

                crate::round_to_five(if row_swap_count % 2 == 0 {
                    result
                } else {
                    -result
                })
            }
        })
    }

    pub fn adjoint(&self) -> Result<Self, MatrixError> {
        if !self.is_square() {
            return Err(MatrixError::NonSquareMatrix);
        }

        Ok(match self.rows {
            1 => Matrix::new_with_data(vec![vec![1.0]]).unwrap(),
            2 => Matrix::new_with_data(vec![
                vec![self.data[1][1], -self.data[0][1]],
                vec![-self.data[1][0], self.data[0][0]],
            ])
            .unwrap(),
            _ => {
                let mut cofactor_matrix = Matrix::new_zero_matrix(self.rows, self.columns).unwrap();
                self.data.iter().enumerate().for_each(|(row_index, row)| {
                    row.iter().enumerate().for_each(|(column_index, _num)| {
                        let mut sub_matrix_data = vec![vec![0.0; self.columns - 1]; self.rows - 1];

                        self.data.iter().enumerate().for_each(|(i, row)| {
                            row.iter().enumerate().for_each(|(j, num)| {
                                if i != row_index && j != column_index {
                                    sub_matrix_data[if i < row_index { i } else { i - 1usize }]
                                        [if j < column_index { j } else { j - 1usize }] = *num
                                }
                            })
                        });

                        let sub_matrix = Matrix::new_with_data(sub_matrix_data).unwrap();
                        let minor = sub_matrix.determinant().unwrap();
                        cofactor_matrix.data[row_index][column_index] =
                            if (row_index + column_index) % 2 == 0 {
                                minor
                            } else {
                                -minor
                            };
                    })
                });

                cofactor_matrix.transpose()
            }
        })
    }

    pub fn inverse(&self) -> Result<Self, MatrixError> {
        if !self.is_square() {
            return Err(MatrixError::NonSquareMatrix);
        }

        let determinant = self.determinant().unwrap();
        if let 0.0 = determinant {
            return Err(MatrixError::SingularMatrix);
        }

        Ok((1.0 / determinant) * &self.adjoint().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;

    #[test]
    fn transpose() {
        assert_eq!(
            Matrix::new_with_data(vec![vec![7.2, 9.3], vec![13.8, 2.7], vec![5.1, 6.4]]).unwrap(),
            test_utils::generic_examples()[0].transpose()
        )
    }

    #[test]
    fn trace() {
        assert_eq!(
            MatrixError::NonSquareMatrix,
            test_utils::generic_examples()[0].trace().unwrap_err()
        );

        let [matrix_1x1, matrix_2x2, matrix_3x3, matrix_5x5] = test_utils::square_examples();
        assert_eq!(2.5, matrix_1x1.trace().unwrap());
        assert_eq!(11.2, matrix_2x2.trace().unwrap());
        assert_eq!(4.5, matrix_3x3.trace().unwrap());
        assert_eq!(6.1, matrix_5x5.trace().unwrap());
    }

    #[test]
    fn determinant() {
        assert_eq!(
            MatrixError::NonSquareMatrix,
            test_utils::generic_examples()[0].determinant().unwrap_err()
        );

        let [matrix_1x1, matrix_2x2, matrix_3x3, matrix_5x5] = test_utils::square_examples();

        assert_eq!(2.5, matrix_1x1.determinant().unwrap());
        assert_eq!(26.51, matrix_2x2.determinant().unwrap());
        assert_eq!(492.164, matrix_3x3.determinant().unwrap());
        assert_eq!(-2204.89804, matrix_5x5.determinant().unwrap());
        assert_eq!(
            0.0,
            Matrix::new_zero_matrix(3, 3)
                .unwrap()
                .determinant()
                .unwrap()
        );
    }

    #[test]
    fn adjoint() {
        assert_eq!(
            MatrixError::NonSquareMatrix,
            test_utils::generic_examples()[0].adjoint().unwrap_err()
        );

        let [matrix_1x1, matrix_2x2, matrix_3x3, _] = test_utils::square_examples();

        assert_eq!(
            Matrix::new_with_data(vec![vec![1.0]]).unwrap(),
            matrix_1x1.adjoint().unwrap()
        );

        assert_eq!(
            Matrix::new_with_data(vec![vec![6.7, -2.8], vec![-1.3, 4.5]]).unwrap(),
            matrix_2x2.adjoint().unwrap()
        );

        assert_eq!(
            Matrix::new_with_data(vec![
                vec![-87.28, 35.29, 64.24],
                vec![35.76, -18.97, 14.28],
                vec![93.88, 31.4, -78.12],
            ])
            .unwrap(),
            matrix_3x3.adjoint().unwrap()
        );
    }

    #[test]
    fn inverse() {
        assert_eq!(
            MatrixError::NonSquareMatrix,
            test_utils::generic_examples()[0].inverse().unwrap_err()
        );

        assert_eq!(
            MatrixError::SingularMatrix,
            Matrix::new_with_data(vec![vec![1.0, 2.0], vec![2.0, 4.0]])
                .unwrap()
                .inverse()
                .unwrap_err()
        );

        let [matrix_1x1, matrix_2x2, matrix_3x3, _] = test_utils::square_examples();

        assert_eq!(
            Matrix::new_with_data(vec![vec![0.4]]).unwrap(),
            matrix_1x1.inverse().unwrap()
        );

        assert_eq!(
            Matrix::new_with_data(vec![vec![0.25273, -0.10562], vec![-0.04904, 0.16975]]).unwrap(),
            matrix_2x2.inverse().unwrap()
        );

        assert_eq!(
            Matrix::new_with_data(vec![
                vec![-0.17734, 0.07170, 0.13053],
                vec![0.07266, -0.03854, 0.02901],
                vec![0.19075, 0.06380, -0.15873]
            ])
            .unwrap(),
            matrix_3x3.inverse().unwrap()
        );
    }
}

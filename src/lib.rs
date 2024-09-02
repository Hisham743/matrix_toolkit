use std::fmt::{self, Display, Formatter};

fn round_to_five(num: f64) -> f64 {
    (num * 100_000.0).round() / 100_000.0
}

mod operations;
mod properties;
mod special_operations;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    rows: usize,
    columns: usize,
    data: Vec<Vec<f64>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MatrixError {
    ZeroDimension,
    InconsistentColumnSize,
    DimensionMismatch,
    NonSquareMatrix,
    SingularMatrix,
    IndexOutOfBounds,
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let max_column_lengths: Vec<usize> = (0..self.columns)
            .map(|i| {
                self.get_column(i)
                    .unwrap()
                    .iter()
                    .map(|x| x.to_string().len())
                    .max()
                    .expect("Getting maximum column length")
            })
            .collect();

        self.data.iter().try_for_each(|row| {
            row.iter().enumerate().try_for_each(|(column, num)| {
                let total_pad_len = max_column_lengths[column] - num.to_string().len();
                let right_pad_len = total_pad_len / 2;
                let left_pad_len = total_pad_len - right_pad_len + 1;
                write!(
                    f,
                    "{}{num}{}",
                    " ".repeat(left_pad_len),
                    " ".repeat(right_pad_len)
                )
            })?;

            writeln!(f)
        })
    }
}

impl Matrix {
    pub fn new_zero_matrix(rows: usize, columns: usize) -> Result<Self, MatrixError> {
        if rows == 0 || columns == 0 {
            return Err(MatrixError::ZeroDimension);
        }

        Ok(Self {
            rows,
            columns,
            data: vec![vec![0.0; columns]; rows],
        })
    }

    pub fn new_with_data(data: Vec<Vec<f64>>) -> Result<Self, MatrixError> {
        if data.iter().any(|row| data[0].len() != row.len()) {
            return Err(MatrixError::InconsistentColumnSize);
        };

        Ok(Self {
            rows: data.len(),
            columns: if data.is_empty() { 0 } else { data[0].len() },
            data,
        })
    }

    pub fn new_diagonal_matrix(diagonal_elements: &Vec<f64>) -> Result<Self, MatrixError> {
        if diagonal_elements.is_empty() {
            return Err(MatrixError::ZeroDimension);
        }

        let mut diagonal_matrix =
            Matrix::new_zero_matrix(diagonal_elements.len(), diagonal_elements.len()).unwrap();

        diagonal_elements
            .iter()
            .enumerate()
            .for_each(|(i, num)| diagonal_matrix.data[i][i] = *num);

        Ok(diagonal_matrix)
    }

    pub fn new_scalar_matrix(scalar: f64, size: usize) -> Result<Self, MatrixError> {
        if let 0 = size {
            return Err(MatrixError::ZeroDimension);
        }

        Ok(Self::new_diagonal_matrix(&vec![scalar; size]).unwrap())
    }

    pub fn nth_identity(n: usize) -> Result<Self, MatrixError> {
        if let 0 = n {
            return Err(MatrixError::ZeroDimension);
        }

        Ok(Self::new_scalar_matrix(1.0, n).unwrap())
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn get_data(&self) -> Vec<Vec<f64>> {
        self.data.clone()
    }

    pub fn get_row(&self, row: usize) -> Result<Vec<f64>, MatrixError> {
        if row >= self.rows {
            return Err(MatrixError::IndexOutOfBounds);
        }

        Ok(self.data[row].clone())
    }

    pub fn get_column(&self, column: usize) -> Result<Vec<f64>, MatrixError> {
        if column >= self.columns {
            return Err(MatrixError::IndexOutOfBounds);
        }

        Ok(self.data.iter().map(|row| row[column]).collect())
    }

    pub fn get_element(&self, row: usize, column: usize) -> Result<f64, MatrixError> {
        if row >= self.rows || column >= self.columns {
            return Err(MatrixError::IndexOutOfBounds);
        }

        Ok(self.data[row][column])
    }

    pub fn set_data(&mut self, data: Vec<Vec<f64>>) -> Result<(), MatrixError> {
        if data.len() != self.rows || data[0].len() != self.columns {
            return Err(MatrixError::DimensionMismatch);
        }

        if data.iter().any(|row| data[0].len() != row.len()) {
            return Err(MatrixError::InconsistentColumnSize);
        };

        self.data = data;
        Ok(())
    }

    pub fn set_row(&mut self, row: usize, data: Vec<f64>) -> Result<(), MatrixError> {
        if row >= self.rows {
            return Err(MatrixError::IndexOutOfBounds);
        }

        if data.len() != self.columns {
            return Err(MatrixError::DimensionMismatch);
        }

        self.data[row] = data;
        Ok(())
    }

    pub fn set_column(&mut self, column: usize, data: Vec<f64>) -> Result<(), MatrixError> {
        if column >= self.columns {
            return Err(MatrixError::IndexOutOfBounds);
        }

        if data.len() != self.rows {
            return Err(MatrixError::DimensionMismatch);
        }

        data.iter()
            .enumerate()
            .for_each(|(row, num)| self.data[row][column] = *num);
        Ok(())
    }

    pub fn set_element(&mut self, row: usize, column: usize, data: f64) -> Result<(), MatrixError> {
        if column >= self.columns || row >= self.rows {
            return Err(MatrixError::IndexOutOfBounds);
        }

        self.data[row][column] = data;
        Ok(())
    }

    fn element_wise_operation<F>(&self, rhs: &Self, operation: F) -> Result<Self, MatrixError>
    where
        F: Fn(f64, f64) -> f64,
    {
        if self.rows != rhs.rows || self.columns != rhs.columns {
            Err(MatrixError::DimensionMismatch)
        } else {
            Ok(Self::new_with_data(
                self.data
                    .iter()
                    .zip(rhs.data.iter())
                    .map(|(row1, row2)| {
                        row1.iter()
                            .zip(row2.iter())
                            .map(|(num1, num2)| round_to_five(operation(*num1, *num2)))
                            .collect()
                    })
                    .collect(),
            )
            .unwrap())
        }
    }
}

#[allow(dead_code)]
mod test_utils {
    use super::*;

    pub(crate) fn generic_examples() -> [Matrix; 3] {
        [
            Matrix::new_with_data(vec![vec![7.2, 13.8, 5.1], vec![9.3, 2.7, 6.4]]).unwrap(),
            Matrix::new_with_data(vec![vec![1.5, 8.9, 3.2], vec![6.7, 11.3, 4.8]]).unwrap(),
            Matrix::new_with_data(vec![
                vec![5.6, 9.8],
                vec![2.9, 7.4],
                vec![11.2, 3.1],
                vec![6.3, 8.7],
            ])
            .unwrap(),
        ]
    }

    pub(crate) fn square_examples() -> [Matrix; 4] {
        [
            Matrix::new_with_data(vec![vec![2.5]]).unwrap(),
            Matrix::new_with_data(vec![vec![4.5, 2.8], vec![1.3, 6.7]]).unwrap(),
            Matrix::new_with_data(vec![
                vec![2.1, 9.7, 3.5],
                vec![8.4, 1.6, 7.2],
                vec![5.9, 12.3, 0.8],
            ])
            .unwrap(),
            Matrix::new_with_data(vec![
                vec![0.0, 7.1, 0.5, 9.3, 2.8],
                vec![6.4, 1.9, 8.7, 4.2, 5.6],
                vec![0.3, 9.8, 2.1, 7.5, 3.9],
                vec![5.7, 3.6, 8.2, 1.4, 6.0],
                vec![9.1, 4.5, 2.6, 7.8, 0.7],
            ])
            .unwrap(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils;

    #[test]
    fn new_zero_matrix() {
        let matrix1 = Matrix::new_zero_matrix(2, 3).unwrap();
        assert_eq!(
            Matrix {
                rows: 2,
                columns: 3,
                data: vec![vec![0.0; 3]; 2]
            },
            matrix1
        );

        assert_eq!(
            MatrixError::ZeroDimension,
            Matrix::new_zero_matrix(0, 1).unwrap_err()
        );

        assert_eq!(
            MatrixError::ZeroDimension,
            Matrix::new_zero_matrix(1, 0).unwrap_err()
        );

        assert_eq!(
            MatrixError::ZeroDimension,
            Matrix::new_zero_matrix(0, 0).unwrap_err()
        );
    }

    #[test]
    fn new_with_data() {
        assert_eq!(
            Matrix {
                rows: 2,
                columns: 3,
                data: vec![vec![4.5, 54.6, 0.0], vec![2.4, 10.4, 1.8]]
            },
            Matrix::new_with_data(vec![vec![4.5, 54.6, 0.0], vec![2.4, 10.4, 1.8]]).unwrap()
        );

        assert_eq!(
            MatrixError::InconsistentColumnSize,
            Matrix::new_with_data(vec![vec![4.5, 54.6, 0.0], vec![2.4, 10.4]]).unwrap_err()
        );
    }

    #[test]
    fn get_data() {
        let [example_matrix, _, _] = test_utils::generic_examples();

        assert_eq!(
            vec![vec![7.2, 13.8, 5.1], vec![9.3, 2.7, 6.4]],
            example_matrix.get_data()
        );

        assert_eq!(
            MatrixError::IndexOutOfBounds,
            example_matrix.get_row(2).unwrap_err()
        );
        assert_eq!(vec![9.3, 2.7, 6.4], example_matrix.get_row(1).unwrap());

        assert_eq!(
            MatrixError::IndexOutOfBounds,
            example_matrix.get_column(3).unwrap_err()
        );
        assert_eq!(vec![13.8, 2.7], example_matrix.get_column(1).unwrap());

        assert_eq!(
            MatrixError::IndexOutOfBounds,
            example_matrix.get_element(2, 1).unwrap_err()
        );
        assert_eq!(
            MatrixError::IndexOutOfBounds,
            example_matrix.get_element(1, 3).unwrap_err()
        );
        assert_eq!(2.7, example_matrix.get_element(1, 1).unwrap());
    }

    #[test]
    fn set_data() {
        let [mut example_matrix, _, _] = test_utils::generic_examples();

        assert_eq!(
            MatrixError::DimensionMismatch,
            example_matrix
                .set_data(vec![vec![1.0], vec![2.3], vec![5.1]])
                .unwrap_err()
        );

        assert_eq!(
            MatrixError::InconsistentColumnSize,
            example_matrix
                .set_data(vec![vec![1.0, 4.3, 7.9], vec![2.6, 11.9]])
                .unwrap_err()
        );

        let data = vec![vec![2.6, 7.1, 0.0], vec![3.5, 7.1, 9.0]];
        example_matrix.set_data(data.clone()).unwrap();
        assert_eq!(Matrix::new_with_data(data).unwrap(), example_matrix);
    }

    #[test]
    fn set_row() {
        let [mut matrix_2x3, _, _] = test_utils::generic_examples();

        assert_eq!(
            MatrixError::IndexOutOfBounds,
            matrix_2x3.set_row(3, vec![1.0, 2.3, 5.6]).unwrap_err()
        );

        assert_eq!(
            MatrixError::DimensionMismatch,
            matrix_2x3.set_row(1, vec![1.0, 2.3]).unwrap_err()
        );

        let data = vec![2.6, 7.1, 0.0];
        matrix_2x3.set_row(1, data.clone()).unwrap();
        assert_eq!(data, matrix_2x3.data[1]);
    }

    #[test]
    fn set_column() {
        let [mut matrix_2x3, _, _] = test_utils::generic_examples();

        assert_eq!(
            MatrixError::IndexOutOfBounds,
            matrix_2x3.set_column(3, vec![1.0, 2.3, 5.6]).unwrap_err()
        );

        assert_eq!(
            MatrixError::DimensionMismatch,
            matrix_2x3
                .set_column(1, vec![1.0, 2.3, 34.8, 0.0])
                .unwrap_err()
        );

        let data = vec![2.6, 7.1];
        matrix_2x3.set_column(1, data.clone()).unwrap();
        assert!(matrix_2x3
            .data
            .iter()
            .enumerate()
            .all(|(index, row)| row[1] == data[index]));
    }

    #[test]
    fn set_element() {
        let [mut matrix_2x3, _, _] = test_utils::generic_examples();

        assert_eq!(
            MatrixError::IndexOutOfBounds,
            matrix_2x3.set_element(3, 1, 11.1).unwrap_err()
        );

        assert_eq!(
            MatrixError::IndexOutOfBounds,
            matrix_2x3.set_element(1, 3, 11.1).unwrap_err()
        );

        matrix_2x3.set_element(1, 1, 11.1).unwrap();
        assert_eq!(11.1, matrix_2x3.data[1][1]);
    }

    #[test]
    fn diagonal_matrix() {
        assert_eq!(
            MatrixError::ZeroDimension,
            Matrix::new_diagonal_matrix(&vec![]).unwrap_err()
        );

        assert_eq!(
            Matrix {
                rows: 4,
                columns: 4,
                data: vec![
                    vec![5.5, 0.0, 0.0, 0.0],
                    vec![0.0, 7.8, 0.0, 0.0],
                    vec![0.0, 0.0, 3.2, 0.0],
                    vec![0.0, 0.0, 0.0, 11.0],
                ]
            },
            Matrix::new_diagonal_matrix(&vec![5.5, 7.8, 3.2, 11.0]).unwrap()
        )
    }

    #[test]
    fn scalar_matrix() {
        assert_eq!(
            MatrixError::ZeroDimension,
            Matrix::new_scalar_matrix(6.5, 0).unwrap_err()
        );

        assert_eq!(
            Matrix {
                rows: 4,
                columns: 4,
                data: vec![
                    vec![5.5, 0.0, 0.0, 0.0],
                    vec![0.0, 5.5, 0.0, 0.0],
                    vec![0.0, 0.0, 5.5, 0.0],
                    vec![0.0, 0.0, 0.0, 5.5],
                ]
            },
            Matrix::new_scalar_matrix(5.5, 4).unwrap()
        )
    }

    #[test]
    fn nth_identity() {
        assert_eq!(
            MatrixError::ZeroDimension,
            Matrix::nth_identity(0).unwrap_err()
        );

        assert_eq!(
            Matrix {
                rows: 3,
                columns: 3,
                data: vec![
                    vec![1.0, 0.0, 0.0],
                    vec![0.0, 1.0, 0.0],
                    vec![0.0, 0.0, 1.0]
                ]
            },
            Matrix::nth_identity(3).unwrap()
        )
    }
}

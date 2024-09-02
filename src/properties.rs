use crate::Matrix;

impl Matrix {
    pub fn is_square(&self) -> bool {
        self.rows == self.columns
    }

    pub fn is_symmetric(&self) -> bool {
        if !self.is_square() {
            return false;
        }
        self == &self.transpose()
    }

    pub fn is_skew_symmetric(&self) -> bool {
        if !self.is_square() {
            return false;
        }
        *self == -&self.transpose()
    }

    pub fn is_diagonal(&self) -> bool {
        if !self.is_square() {
            return false;
        }

        self.data.iter().enumerate().all(|(row_index, row)| {
            row.iter()
                .enumerate()
                .all(|(column_index, num)| *num == 0.0 || row_index == column_index)
        })
    }

    pub fn is_scalar(&self) -> bool {
        if !self.is_square() || self.data.is_empty() {
            return false;
        }

        let first_element = self.data[0][0];
        self.data.iter().enumerate().all(|(row_index, row)| {
            row.iter().enumerate().all(|(column_index, num)| {
                *num == if row_index == column_index {
                    first_element
                } else {
                    0.0
                }
            })
        })
    }

    pub fn is_identity(&self) -> bool {
        if self.is_scalar() && !self.data.is_empty() {
            self.data[0][0] == 1.0
        } else {
            false
        }
    }

    pub fn is_zero(&self) -> bool {
        self.data
            .iter()
            .all(|row| row.iter().all(|num| *num == 0.0))
    }

    pub fn is_singular(&self) -> bool {
        self.determinant()
            .map_or(false, |determinant| determinant == 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;

    #[test]
    fn square() {
        assert!(!test_utils::generic_examples()[0].is_square());
        assert!(test_utils::square_examples()
            .iter()
            .all(|matrix| matrix.is_square()));
    }

    #[test]
    fn symmetric() {
        assert!(!test_utils::square_examples()[2].is_symmetric());

        let symmetric_matrix = Matrix::new_with_data(vec![
            vec![9.5, 2.3, 3.5],
            vec![2.3, -1.0, -8.5],
            vec![3.5, -8.5, 0.0],
        ])
        .unwrap();
        assert!(symmetric_matrix.is_symmetric());
    }

    #[test]
    fn skew_symmetric() {
        assert!(!test_utils::square_examples()[2].is_skew_symmetric());

        let skew_symmetric_matrix = Matrix::new_with_data(vec![
            vec![0.0, 2.3, 3.5],
            vec![-2.3, 0.0, -8.5],
            vec![-3.5, 8.5, 0.0],
        ])
        .unwrap();
        assert!(skew_symmetric_matrix.is_skew_symmetric());
    }

    #[test]
    fn diagonal() {
        assert!(!test_utils::square_examples()[2].is_diagonal());

        let diagonal_matrix = Matrix::new_with_data(vec![
            vec![1.5, 0.0, 0.0, 0.0],
            vec![0.0, 3.2, 0.0, 0.0],
            vec![0.0, 0.0, 6.7, 0.0],
            vec![0.0, 0.0, 0.0, 9.1],
        ])
        .unwrap();
        assert!(diagonal_matrix.is_diagonal());
    }

    #[test]
    fn scalar() {
        let diagonal_matrix = Matrix::new_with_data(vec![
            vec![1.5, 0.0, 0.0, 0.0],
            vec![0.0, 3.2, 0.0, 0.0],
            vec![0.0, 0.0, 6.7, 0.0],
            vec![0.0, 0.0, 0.0, 9.1],
        ])
        .unwrap();
        assert!(!diagonal_matrix.is_scalar());

        let scalar_matrix = Matrix::new_with_data(vec![
            vec![2.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0],
            vec![0.0, 0.0, 2.0],
        ])
        .unwrap();
        assert!(scalar_matrix.is_scalar());
    }

    #[test]
    fn identity() {
        let scalar_matrix = Matrix::new_with_data(vec![
            vec![2.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0],
            vec![0.0, 0.0, 2.0],
        ])
        .unwrap();
        assert!(!scalar_matrix.is_identity());

        let identity_matrix = Matrix::new_with_data(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap();
        assert!(identity_matrix.is_identity());
    }

    #[test]
    fn zero() {
        assert!(!test_utils::generic_examples()[0].is_zero());
        assert!(Matrix::new_with_data(vec![vec![0.0, 0.0]])
            .unwrap()
            .is_zero());
    }

    #[test]
    fn singular() {
        assert!(!test_utils::generic_examples()[0].is_singular());
        assert!(Matrix::new_with_data(vec![vec![1.0, 2.0], vec![2.0, 4.0]])
            .unwrap()
            .is_singular());
    }
}

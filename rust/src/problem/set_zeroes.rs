pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
    let mut first_row = false;
    let mut first_col = false;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 0 {
                if i == 0 {
                    first_row = true;
                }

                if j == 0 {
                    first_col = true;
                }

                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }

    for i in 1..matrix.len() {
        for j in 1..matrix[i].len() {
            if matrix[i][0] == 0 || matrix[0][j] == 0 {
                matrix[i][j] = 0;
            }
        }
    }

    if first_row {
        for cell in matrix[0].iter_mut() {
            *cell = 0;
        }
    }

    if first_col {
        for row in matrix.iter_mut() {
            row[0] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_zeroes() {
        let mut matrix = vec![
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1],
        ];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![
            vec![1, 0, 1],
            vec![0, 0, 0],
            vec![1, 0, 1],
        ]);

        let mut matrix = vec![
            vec![0, 1, 2, 0],
            vec![3, 4, 5, 2],
            vec![1, 3, 1, 5],
        ];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![
            vec![0, 0, 0, 0],
            vec![0, 4, 5, 0],
            vec![0, 3, 1, 0],
        ]);

        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 0, 7, 8],
            vec![0, 10, 11, 12],
            vec![13, 14, 15, 0],
        ];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![
            vec![0, 0, 3, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ]);
    }
}
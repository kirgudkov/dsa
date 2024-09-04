// The idea is to use 0-th row and 0-th col as masks to mark corresponding row/col as "need to be filled with zeros":
// [1,0,1]
// [1,1,1]
// [0,1,1]
// In this example 0-th row marked 1-th col as filled and 0th-col marked 2-th row as filled:
// [1,0,1]
// [1,0,1]
// [0,0,0]
// The only subtle thing is that we should handle cases when mask row/col has zeros initially, se we need to fill them too.

pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
    let mut row_has_zero = false;
    let mut col_has_zero = false;

    // 1. Traverse the entire matrix and find all zeros.
    // Here we only mark out mask row/col.
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 0 {
                // If we meet zero, we mark the position in 0-th row and 0-th col with 0 to deal with them later;
                matrix[i][0] = 0;
                matrix[0][j] = 0;

                // If zero has been met at our mask row/col, we remeber it in order to fill them at the very end; 
                if i == 0 { row_has_zero = true; }
                if j == 0 { col_has_zero = true; }
            }
        }
    }

    // 2. Process everything except mask row/col;
    for i in 1..matrix.len() {
        for j in 1..matrix[i].len() {
            if matrix[i][0] == 0 || matrix[0][j] == 0 {
                matrix[i][j] = 0;
            }
        }
    }

    // 3. After we processed everything, it's time to deal with mask row and col
    if row_has_zero {
        for cell in matrix[0].iter_mut() {
            *cell = 0;
        }
    }

    if col_has_zero {
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
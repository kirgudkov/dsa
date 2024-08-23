pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::with_capacity(mat.len() * mat[0].len());

    let mut i = 0;
    let mut j = 0;

    while result.len() != result.capacity() {
        result.push(mat[i][j]);

        match (i + j) % 2 {
            0 => {
                if j == mat[0].len() - 1 {
                    i += 1;
                } else if i == 0 {
                    j += 1;
                } else {
                    i -= 1;
                    j += 1;
                }
            }
            _ => {
                if i == mat.len() - 1 {
                    j += 1;
                } else if j == 0 {
                    i += 1;
                } else {
                    i += 1;
                    j -= 1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_diagonal_order() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = find_diagonal_order(mat);
        assert_eq!(result, vec![1, 2, 4, 7, 5, 3, 6, 8, 9]);

        let mat = vec![vec![1, 2], vec![3, 4]];
        let result = find_diagonal_order(mat);
        assert_eq!(result, vec![1, 2, 3, 4]);

        let mat = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];
        let result = find_diagonal_order(mat);
        assert_eq!(result, vec![1, 2, 5, 6, 3, 4, 7, 8]);

        let mat = vec![vec![7], vec![9], vec![6]];
        let result = find_diagonal_order(mat);
        assert_eq!(result, vec![7, 9, 6]);

        let mat = vec![vec![1, 2, 3]];
        let result = find_diagonal_order(mat);
        assert_eq!(result, vec![1, 2, 3]);

        let mat = vec![vec![2, 5], vec![8, 4], vec![0, -1]];
        let result = find_diagonal_order(mat);
        assert_eq!(result, vec![2, 5, 8, 0, 4, -1]);
    }
}
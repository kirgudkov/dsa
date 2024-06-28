pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let mut dp = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
    let mut max_side = 0;

    for i in (0..matrix.len()).rev() {
        for j in (0..matrix[0].len()).rev() {
            if matrix[i][j] == '1' {
                dp[i][j] = 1 + dp[i + 1][j + 1].min(dp[i + 1][j].min(dp[i][j + 1]));
                max_side = max_side.max(dp[i][j]);
            }
        }
    }

    max_side * max_side
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximal_square() {
        assert_eq!(
            maximal_square(
                vec![
                    vec!['1', '0', '1', '0', '0'],
                    vec!['1', '0', '1', '1', '1'],
                    vec!['1', '1', '1', '1', '1'],
                    vec!['1', '0', '0', '1', '0'],
                ]
            ),
            4
        );
        assert_eq!(
            maximal_square(
                vec![
                    vec!['0', '1'],
                    vec!['1', '0'],
                ]
            ),
            1
        );
        assert_eq!(
            maximal_square(
                vec![
                    vec!['0'],
                ]
            ),
            0
        );
    }
}
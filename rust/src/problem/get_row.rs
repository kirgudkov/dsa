// https://leetcode.com/problems/pascals-triangle-ii
// Recursive solution with memoization.
pub fn get_row(i: i32) -> Vec<i32> {
    let mut memo = vec![vec![0; (i + 1) as usize]; (i + 1) as usize];
    let mut result = vec![0; (i + 1) as usize];

    for (j, cell) in result.iter_mut().enumerate() {
        *cell = get_cell(i as usize, j, &mut memo);
    }

    result
}

fn get_cell(i: usize, j: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
    if memo[i][j] != 0 {
        return memo[i][j];
    }

    if j == 0 || j == i {
        return 1;
    }

    memo[i][j] = get_cell(i - 1, j - 1, memo) + get_cell(i - 1, j, memo);
    memo[i][j]
}

#[cfg(test)]
mod tests {
    use crate::problem::get_row::get_row;

    #[test]
    fn test_get_row() {
        assert_eq!(get_row(0), vec![1]);
        assert_eq!(get_row(1), vec![1, 1]);
        assert_eq!(get_row(2), vec![1, 2, 1]);
        assert_eq!(get_row(3), vec![1, 3, 3, 1]);

        // Takes ~22 seconds to run on M2 Max without memoization.
        assert_eq!(get_row(29), vec![
            1, 29, 406, 3654, 23751, 118755, 475020, 1560780, 4292145, 10015005, 20030010, 34597290,
            51895935, 67863915, 77558760, 77558760, 67863915, 51895935, 34597290, 20030010, 10015005,
            4292145, 1560780, 475020, 118755, 23751, 3654, 406, 29, 1,
        ]);
    }
}

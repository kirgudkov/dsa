// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix
// Binary search applied on each row, so the tim complexity is O(n logm)
pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;

    fn bs(row: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = row.len() as i32 - 1;

        while l <= r {
            let m = l + (r - l) / 2;

            if row[m as usize] >= 0 {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        l
    }

    for row in grid {
        count += row.len() as i32 - bs(row);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_negatives() {
        assert_eq!(count_negatives(vec![vec![4, 3, 2, -1], vec![3, 2, 1, -1], vec![1, 1, -1, -2], vec![-1, -1, -2, -3]]), 8);
        assert_eq!(count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
        assert_eq!(count_negatives(vec![vec![1, -1], vec![-1, -1]]), 3);
        assert_eq!(count_negatives(vec![vec![-1]]), 1);
    }
}

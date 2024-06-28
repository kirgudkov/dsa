pub fn maximum_score_td(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
    let mut memo = vec![vec![-1; multipliers.len()]; multipliers.len()];

    fn solve(memo: &mut Vec<Vec<i32>>, nums: &[i32], multipliers: &[i32], l: usize, i: usize) -> i32 {
        if i == multipliers.len() {
            return 0;
        }

        if memo[i][l] != -1 {
            return memo[i][l];
        }

        let r = nums.len() - 1 - i + l;

        memo[i][l] = std::cmp::max(
            multipliers[i] * nums[l] + solve(memo, nums, multipliers, l + 1, i + 1),
            multipliers[i] * nums[r] + solve(memo, nums, multipliers, l, i + 1),
        );

        memo[i][l]
    }

    solve(&mut memo, &nums, &multipliers, 0, 0)
}

pub fn maximum_score_bu(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
    let mut dp = vec![vec![0; multipliers.len() + 1]; multipliers.len() + 1];

    for op in (0..multipliers.len()).rev() {
        for left in (0..=op).rev() {
            dp[op][left] = std::cmp::max(
                multipliers[op] * nums[left] + dp[op + 1][left + 1],
                multipliers[op] * nums[nums.len() - 1 - (op - left)] + dp[op + 1][left],
            );
        }
    }

    dp[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_score_td() {
        assert_eq!(maximum_score_td(vec![-5, -3, -3, -2, 7, 1], vec![-10, -5, 3, 4, 6]), 102);
        assert_eq!(maximum_score_td(vec![1, 2, 3], vec![3, 2, 1]), 14);
        assert_eq!(maximum_score_td(vec![4, 1, 3], vec![1, 2]), 11);
        assert_eq!(maximum_score_td(vec![4, 1, 3], vec![1, 2, 3]), 17);
    }

    #[test]
    fn test_maximum_score_bu() {
        assert_eq!(maximum_score_bu(vec![-5, -3, -3, -2, 7, 1], vec![-10, -5, 3, 4, 6]), 102);
        assert_eq!(maximum_score_bu(vec![1, 2, 3], vec![3, 2, 1]), 14);
        assert_eq!(maximum_score_bu(vec![4, 1, 3], vec![1, 2]), 11);
        assert_eq!(maximum_score_bu(vec![4, 1, 3], vec![1, 2, 3]), 17);
    }
}

pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums[0];
    }

    let mut dp = vec![0; nums.len()];
    dp[0] = nums[0];
    dp[1] = nums[0].max(nums[1]);

    for i in 2..nums.len() {
        dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
    }

    dp[nums.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(rob(vec![0]), 0);
        assert_eq!(rob(vec![2, 1]), 2);
    }
}
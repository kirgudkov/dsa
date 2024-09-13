// https://leetcode.com/problems/longest-increasing-subsequence
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = vec![1; nums.len()];

    nums.iter().enumerate().skip(1).for_each(|(i, &curr)| {
        nums.iter().enumerate().take(i).for_each(|(j, &prev)| {
            if curr > prev { dp[i] = dp[i].max(dp[j] + 1) }
        });
    });

    *dp.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis() {
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
}
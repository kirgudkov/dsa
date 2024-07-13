pub fn two_sum_less_than_k_tp(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();

    let mut l = 0;
    let mut r = nums.len() - 1;
    let mut sum = -1;

    while l < r {
        if nums[l] + nums[r] <= k {
            sum = sum.max(nums[l] + nums[r]);
            l += 1;
        } else {
            r -= 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_less_than_k_tp() {
        assert_eq!(two_sum_less_than_k_tp(vec![34, 23, 1, 24, 75, 33, 54, 8], 60), 58);
        assert_eq!(two_sum_less_than_k_tp(vec![10, 20, 30], 15), -1);
    }
}
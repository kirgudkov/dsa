pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();
    // This is safe, because min nums size is 3 by constraints
    let mut closest = nums[0] + nums[1] + nums[2];

    // For each number a, try to find a pair [b, c] in the remaining part of the array 
    // such that (target - (a + b + c)) is the smallest
    for (i, &a) in nums.iter().take(nums.len() - 2).enumerate() {
        let mut l = i + 1;
        let mut r = nums.len() - 1;

        while l < r {
            let sum = a + nums[l] + nums[r];

            if sum < target {
                l += 1;
            } else {
                r -= 1;
            }

            if (target - sum).abs() < (target - closest).abs() {
                closest = sum;
            }
        }
    }

    closest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(three_sum_closest(vec![2, 3, 8, 9, 10], 16), 15);
        assert_eq!(three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2), -2);
        assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
        assert_eq!(three_sum_closest(vec![1, 1, 1, 0], -100), 2);
    }
}

pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();

    let mut closest = nums[0] + nums[1] + nums[2];

    for i in 0..nums.len() {
        let mut l = i + 1;
        let mut r = nums.len() - 1;

        while l < r {
            let sum = nums[i] + nums[l] + nums[r];

            if sum == target {
                return sum;
            }

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
    use crate::problem::misc::three_sum_closest::three_sum_closest;

    #[test]
    fn test_0() {
        assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
        assert_eq!(three_sum_closest(vec![1, 1, 1, 0], -100), 2);
    }
}

// https://leetcode.com/problems/binary-subarrays-with-sum/
pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    fn solve(nums: &[i32], goal: i32) -> i32 {
        let mut l = 0;
        let mut sum = 0;
        let mut count = 0;

        for (r, num) in nums.iter().enumerate() {
            sum += num;

            while l <= r && sum > goal {
                sum -= nums[l];
                l += 1;
            }

            count += r as i32 - l as i32 + 1;
        }

        count
    }

    solve(&nums, goal) - solve(&nums, goal - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(num_subarrays_with_sum(vec![0, 1, 1, 1, 1], 3), 3);
        assert_eq!(num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
        assert_eq!(num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0), 15);
    }
}
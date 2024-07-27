// https://leetcode.com/problems/count-number-of-nice-subarrays/
pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    fn sum(nums: &[i32], k: i32) -> i32 {
        let mut l = 0;
        let mut count = 0;
        let mut res = 0;

        for (r, &num) in nums.iter().enumerate() {
            count += num % 2;

            while count > k {
                count -= nums[l] % 2;
                l += 1;
            }

            res += r - l + 1;
        }

        res as i32
    }

    sum(&nums, k) - sum(&nums, k - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
        assert_eq!(number_of_subarrays(vec![2, 4, 6], 1), 0);
        assert_eq!(number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2), 16);
    }
}
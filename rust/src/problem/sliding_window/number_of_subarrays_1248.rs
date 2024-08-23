// https://leetcode.com/problems/count-number-of-nice-subarrays/
pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let at_most = |k: i32, nums: &[i32]| -> i32 {
        let mut l = 0;
        let mut acc = 0;
        let mut count = 0;

        for (r, &num) in nums.iter().enumerate() {
            acc += num % 2; // ood numbers anlways have reamainder of 1

            while l <= r && acc > k {
                acc -= nums[l] % 2;
                l += 1;
            }

            count += 1 + r - l;
        }

        count as i32
    };

    at_most(k, &nums) - at_most(k - 1, &nums)
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
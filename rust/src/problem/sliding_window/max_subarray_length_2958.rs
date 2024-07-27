// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency/
pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut freq = std::collections::HashMap::with_capacity(nums.len());
    let mut res = 0;
    let mut l = 0;

    for (r, num) in nums.iter().enumerate() {
        *freq.entry(*num).or_insert(0) += 1;

        while *freq.get(num).unwrap() > k {
            *freq.get_mut(&nums[l]).unwrap() -= 1;
            l += 1;
        }

        res = res.max((r - l + 1) as i32);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2), 6);
        assert_eq!(max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2], 1), 2);
        assert_eq!(max_subarray_length(vec![5, 5, 5, 5, 5, 5, 5], 4), 4);
        assert_eq!(max_subarray_length(vec![3, 1, 1], 1), 2);
        assert_eq!(max_subarray_length(vec![1, 2, 2, 1, 3], 1), 3);
    }
}

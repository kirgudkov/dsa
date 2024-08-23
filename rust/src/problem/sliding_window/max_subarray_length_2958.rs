// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency/
pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut freq = std::collections::HashMap::with_capacity(nums.len());
    let mut longest = 0;
    let mut l = 0;

    for (r, &num_r) in nums.iter().enumerate() {
        // With each new item on the right, we increase this item frequency 
        let r_freq = *freq.entry(num_r)
            .and_modify(|e| *e += 1)
            .or_insert(1);

        // Valid window should contain not more than k instances of each number;
        // If we have exceeded the k limit, we got invalid window;
        if r_freq > k {
            // So, before counting the length, we should get back to valid state;
            loop {
                let num_l = nums[l];

                // Remove leftmost item and reduce its frequency;
                l += 1;
                *freq.get_mut(&num_l).unwrap() -= 1;

                // If the leftmost item was the same as the rightmost, 
                // that means we 100% got back to valid window -> stop shrinking; 
                if num_l == num_r {
                    break;
                }
            }
        }

        longest = longest.max(r - l + 1);
    }

    longest as i32
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

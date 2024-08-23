pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut zero_count = 0;
    let mut l = 0;

    for (r, &num) in nums.iter().enumerate() {
        if num == 0 {
            zero_count += 1;
        }

        while zero_count > 1 {
            if nums[l] == 0 {
                zero_count -= 1;
            }
            l += 1;
        }

        result = result.max(r - l + 1);
    }

    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_consecutive_ones() {
        assert_eq!(find_max_consecutive_ones(vec![0]), 1);
        assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 1, 0]), 4);
        assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 4);
        assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 0]), 5);
        assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 0, 1]), 5);
        assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1]), 10);
    }
}
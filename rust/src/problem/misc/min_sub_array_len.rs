pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut min_len = None;
    let mut sum = 0;
    let mut l = 0;

    for (r, &num) in nums.iter().enumerate() {
        sum += num;

        while l <= r && sum >= target {
            min_len = Some(min_len.unwrap_or(usize::MAX).min(r - l + 1));
            sum -= nums[l];
            l += 1;
        }
    }

    min_len.unwrap_or(0) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(min_sub_array_len(20, vec![1, 17, 2, 5, 9, 3, 6, 3, 12]), 3);
        assert_eq!(min_sub_array_len(11, vec![1, 2, 3, 4, 5]), 3);
        assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }
}
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut res: Option<i32> = None;
    let mut r = 0;
    let mut l = 0;
    let mut sum = 0;

    while r < nums.len() {
        sum += nums[r];

        if sum >= target {
            res = Some(res.unwrap_or(i32::MAX).min((r - l + 1) as i32));
            sum -= nums[l];
            sum -= nums[r];
            l += 1;
        } else {
            r += 1;
        }
    }

    res.unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
        assert_eq!(min_sub_array_len(20, vec![1, 17, 2, 5, 9, 3, 6, 3, 12]), 3);
        assert_eq!(min_sub_array_len(11, vec![1, 2, 3, 4, 5]), 3);
    }
}
pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;

    while l < r {
        let m = l + (r - l) / 2;

        if nums[m + 1] == nums[m] {
            if (r - m) % 2 == 0 {
                l = m + 2;
            } else {
                r = m - 1;
            }
        } else if nums[m - 1] == nums[m] {
            if (r - m) % 2 == 0 {
                r = m - 2;
            } else {
                l = m + 1;
            }
        } else {
            return nums[m];
        }
    }

    nums[l]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
        assert_eq!(single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]), 10);
        assert_eq!(single_non_duplicate(vec![1, 1, 2]), 2);
        assert_eq!(single_non_duplicate(vec![1, 2, 2]), 1);
        assert_eq!(single_non_duplicate(vec![1]), 1);
        assert_eq!(single_non_duplicate(vec![1, 1, 2, 2, 3]), 3);
        assert_eq!(single_non_duplicate(vec![1, 1, 2, 2, 4, 4, 5, 5, 9]), 9);
    }
}
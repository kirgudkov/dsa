pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l: i32 = 0;
    let mut r: i32 = (nums.len() - 1) as i32;

    while l <= r {
        let m = l + (r - l) / 2;

        if nums[m as usize] == target {
            return m;
        }

        if target < nums[m as usize] {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12, 15], 12), 5);
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12, 15], 17), -1);
        assert_eq!(binary_search(vec![5], 5), 0);
    }
}
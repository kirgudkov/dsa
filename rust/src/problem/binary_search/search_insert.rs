pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0i32;
    let mut r = nums.len() as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;

        if nums[m as usize] == target {
            return m;
        }

        if target > nums[m as usize] {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = (nums.len() - 1) as i32;

    while l <= r {
        let mid = l + (r - l) / 2;

        if nums[mid as usize] == target {
            return mid;
        }

        // left part is sorted
        if nums[l as usize] <= nums[mid as usize] {
            // target is in the left part
            if nums[l as usize] <= target && target < nums[mid as usize] {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
            // target is in between mid and right
        } else if nums[mid as usize] < target && target <= nums[r as usize] {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test() {
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(search(vec![1], 0), -1);
        assert_eq!(search(vec![12, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 1, 2, 3, 4, 5, 6, 7, 8], 6), 16);
    }
}
// https://leetcode.com/problems/search-in-rotated-sorted-array
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0i32;
    let mut r = nums.len() as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;

        if nums[m as usize] == target {
            return m;
        }

        // Since array is rotated, there are two sorted arrays at both sides of pivot point;
        // Thus, by picking mid, we should always get at least one sorted array on the left or right;
        // We need to figure out which side is sorted and then we can safely check if target is in this part;
        if nums[l as usize] <= nums[m as usize] { // left part is sorted;
            if target >= nums[l as usize] && target < nums[m as usize] { // if target is in the left part
                r = m - 1;
            } else {
                l = m + 1;
            }
        } else if target > nums[m as usize] && target <= nums[r as usize] { // if target is in between mid and right
            l = m + 1;
        } else {
            r = m - 1;
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
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array
// TC: O(logN). Binary search is used to find the minimum element.
// SC: O(1). Constant space is used.
// Approach: compare the mid-element with its left and right neighbours.
// If the mid-element is greater than its right neighbour, then the right neighbour is the minimum element.
// If the mid-element is less than its left neighbour, then the mid-element is the minimum element.
// Otherwise, adjust left or right pointer.
pub fn find_min(nums: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, nums.len() - 1);

    if nums[l] < nums[r] {
        return nums[l];
    }

    while l <= r {
        let m = l + (r - l) / 2;

        if m + 1 < nums.len() && nums[m] > nums[m + 1] {
            return nums[m + 1];
        }

        if nums[m] <= nums[m.saturating_sub(1)] {
            return nums[m];
        }

        if nums[m] > nums[l] {
            l = m;
        } else {
            r = m;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_min_test() {
        assert_eq!(find_min(vec![2, 1]), 1);
        assert_eq!(find_min(vec![2, 3, 4, 5, 1]), 1);
        assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
        assert_eq!(find_min(vec![1]), 1);
    }
}
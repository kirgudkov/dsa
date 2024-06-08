// https://leetcode.com/problems/find-peak-element
// TC: O(logN). Binary search is used to find the peak element.
// SC: O(1). Constant space is used.
// Approach: Binary search is used to find the peak element. If the mid-element is greater than its neighbours, then it is the peak element.
// If the mid-element is less than its right neighbour, then the peak element is on the right side of the mid-element.
// If the mid-element is less than its left neighbour, then the peak element is on the left side of the mid-element.
pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }

    let mut l = 0;
    let mut r = nums.len() - 1;

    if nums[l] > nums[l + 1] {
        return l as i32;
    }

    if nums[r] > nums[r - 1] {
        return r as i32;
    }

    while l < r {
        let m = l + (r - l) / 2;

        if nums[m] > nums[m + 1] && nums[m] > nums[m - 1] {
            return m as i32;
        }

        if nums[m] < nums[m + 1] {
            l = m;
        } else {
            r = m;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_peak_element_test() {
        assert_eq!(find_peak_element(vec![3, 4, 3, 2, 1]), 1);
        assert_eq!(find_peak_element(vec![6, 5, 4, 3, 2, 3, 2]), 0);
        assert_eq!(find_peak_element(vec![1, 2, 3, 1]), 2);
        assert_eq!(find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
        assert_eq!(find_peak_element(vec![1]), 0);
    }
}
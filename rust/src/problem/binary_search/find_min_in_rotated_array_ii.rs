// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii
// TC is O(n) in the worst case. SC is O(1).
pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;

    while l < r {
        let m = l + (r - l) / 2;

        if nums[m] > nums[r] {
            l = m + 1;
        } else if nums[m] < nums[l] {
            r = m;
        // For situation like this: [10, 1, 10, 10, 10]
        //                  l       m       r
        // we can't decide which side to go, so the only choice we have is to reduce r by 1
        } else {
            r -= 1;
        }
    };

    nums[l]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_min(vec![10, 1, 10, 10, 10]), 1);
        assert_eq!(find_min(vec![1]), 1);
        assert_eq!(find_min(vec![1, 1]), 1);
        assert_eq!(find_min(vec![1, 3, 5]), 1);
        assert_eq!(find_min(vec![2, 2, 2, 2, 0, 1]), 0);
        assert_eq!(find_min(vec![3, 1, 3]), 1);
        assert_eq!(find_min(vec![1, 3, 3]), 1);
        assert_eq!(find_min(vec![5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 1, 2, 3, 4]), 1);
    }
}
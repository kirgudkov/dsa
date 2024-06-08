// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array
// TC: O(logN)
// SC: O(1)
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![-1, -1];
    }

    let find_left = || -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;
        let mut result = -1;

        while l <= r {
            let m = l + (r - l) / 2;
            if nums[m as usize] >= target {
                if nums[m as usize] == target {
                    result = m;
                }
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        result
    };

    let find_right = || -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;
        let mut result = -1;

        while l <= r {
            let m = l + (r - l) / 2;
            if nums[m as usize] <= target {
                if nums[m as usize] == target {
                    result = m;
                }
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        result
    };

    vec![find_left(), find_right()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
        assert_eq!(search_range(vec![], 0), vec![-1, -1]);
        assert_eq!(search_range(vec![1], 0), vec![-1, -1]);
        assert_eq!(search_range(vec![1], 1), vec![0, 0]);
        assert_eq!(search_range(vec![2, 2], 1), vec![-1, -1]);
        assert_eq!(search_range(vec![1, 4], 4), vec![1, 1]);
    }
}
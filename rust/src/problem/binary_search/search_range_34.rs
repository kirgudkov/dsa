// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array
// TC: O(logN)
// SC: O(1)
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let edge = |side: char| -> i32 {
        let (mut l, mut r, mut pos) = (0, nums.len() as i32 - 1, -1);

        while l <= r {
            let m = l + (r - l) / 2;

            match side {
                'l' => {
                    if nums[m as usize] >= target {
                        if nums[m as usize] == target {
                            pos = m;
                        }
                        r = m - 1;
                    } else {
                        l = m + 1;
                    }
                }
                _ => {
                    if nums[m as usize] <= target {
                        if nums[m as usize] == target {
                            pos = m;
                        }
                        l = m + 1;
                    } else {
                        r = m - 1;
                    }
                }
            }
        }

        pos
    };

    vec![edge('l'), edge('r')]
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
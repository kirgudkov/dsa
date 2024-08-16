use std::collections::HashSet;

// https://leetcode.com/problems/intersection-of-two-arrays

// TC is O(n log n) + O(m log n);
pub fn intersection_bs(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    nums1.sort_unstable();

    nums2.iter()
        .filter(|&x| bs(&nums1, x).is_some())
        .copied().collect::<HashSet<i32>>().iter()
        .copied().collect::<Vec<i32>>()
}

fn bs(vec: &[i32], x: &i32) -> Option<usize> {
    let mut l = 0;
    let mut r = vec.len() - 1;

    while l <= r {
        let m = l + (r - l) / 2;

        if vec[m] == *x {
            return Some(m);
        }

        if vec[m] > *x {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    None
}

// TC is O(n + m)
pub fn intersection_hs(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut set: HashSet<i32> = nums1.into_iter().collect();

    nums2.iter()
        .filter(|&x| set.remove(x))
        .copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_bs() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(intersection_bs(nums1, nums2), vec![2]);
    }

    #[test]
    fn test_intersection_hs() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(intersection_hs(nums1, nums2), vec![2]);
    }
}
use std::collections::HashSet;

// https://leetcode.com/problems/intersection-of-two-arrays

// TC is O(n log n) + O(m log n);
pub fn intersection_bs(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut res = HashSet::new();
    nums1.sort_unstable();

    for item in nums2.iter() {
        if let Some(i) = bs(&nums1, item) {
            res.insert(nums1[i]);
        }
    }

    res.iter().copied().collect()
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
    let mut res = vec![];

    for num in nums2 {
        if set.remove(&num) {
            res.push(num);
        }
    }

    res
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
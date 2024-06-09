// https://leetcode.com/problems/intersection-of-two-arrays-ii
// Two pointers approach. TC is O(n log n + m log m)
pub fn intersection_tp(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];

    nums1.sort_unstable(); // O(n log n)
    nums2.sort_unstable(); // O(m log m)

    let mut a = 0;
    let mut b = 0;

    while a < nums1.len() && b < nums2.len() { // O(min(n, m))
        match nums1[a].cmp(&nums2[b]) {
            std::cmp::Ordering::Equal => {
                res.push(nums1[a]);
                a += 1;
                b += 1;
            }
            std::cmp::Ordering::Less => {
                a += 1;
            }
            std::cmp::Ordering::Greater => {
                b += 1;
            }
        }
    }

    res
}

// HashMap approach. TC is O(n + m)
pub fn intersection_hm(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut freq = std::collections::HashMap::new();
    let mut res = vec![];

    for num in nums1.iter() {
        *freq.entry(num).or_insert(0) += 1;
    }

    for num in nums2.iter() {
        if let Some(count) = freq.get_mut(num) {
            if *count > 0 {
                res.push(*num);
                *count -= 1;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_tp() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(intersection_tp(nums1, nums2), vec![2, 2]);

        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        assert_eq!(intersection_tp(nums1, nums2), vec![4, 9]);

        let nums1 = vec![3, 1, 2];
        let nums2 = vec![1, 1];
        assert_eq!(intersection_tp(nums1, nums2), vec![1]);
    }

    fn test_intersection_hm() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(intersection_hm(nums1, nums2), vec![2, 2]);

        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        assert_eq!(intersection_hm(nums1, nums2), vec![4, 9]);

        let nums1 = vec![3, 1, 2];
        let nums2 = vec![1, 1];
        assert_eq!(intersection_hm(nums1, nums2), vec![1]);
    }
}
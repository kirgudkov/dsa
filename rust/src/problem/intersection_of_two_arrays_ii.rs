// https://leetcode.com/problems/intersection-of-two-arrays-ii
// Two pointers approach. TC is O(n log n + m log m)
pub fn intersection_tp(mut nums_1: Vec<i32>, mut nums_2: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];

    nums_1.sort_unstable(); // O(n log n)
    nums_2.sort_unstable(); // O(m log m)

    let mut a = 0;
    let mut b = 0;

    while a < nums_1.len() && b < nums_2.len() { // O(min(n, m))
        match nums_1[a].cmp(&nums_2[b]) {
            std::cmp::Ordering::Less => a += 1,
            std::cmp::Ordering::Greater => b += 1,
            std::cmp::Ordering::Equal => {
                res.push(nums_1[a]);
                a += 1;
                b += 1;
            }
        }
    }

    res
}

// HashMap approach. TC is O(n + m)
pub fn intersection_hm(nums_1: Vec<i32>, nums_2: Vec<i32>) -> Vec<i32> {
    let mut freq = std::collections::HashMap::new();
    let mut res = vec![];

    nums_1.iter().for_each(|n| {
        *freq.entry(n).or_insert(0) += 1;
    });

    nums_2.iter().for_each(|n| {
        if let Some(count) = freq.get_mut(n) {
            if *count > 0 {
                res.push(*n);
                *count -= 1;
            }
        }
    });

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
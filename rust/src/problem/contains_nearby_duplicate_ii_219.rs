use std::collections::{HashMap, HashSet};

// https://leetcode.com/problems/contains-duplicate-ii
pub fn contains_nearby_duplicate_1(nums: Vec<i32>, k: i32) -> bool {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if let Some(last_i) = map.get_mut(num) {
            if (*last_i - i as i32).abs() <= k {
                return true;
            } else {
                *last_i = i as i32;
            }
        } else {
            map.insert(*num, i as i32);
        }
    }

    false
}

// Slightly better solution with two pointers + we're keeping set size <= k
pub fn contains_nearby_duplicate_2(nums: Vec<i32>, k: i32) -> bool {
    if k == 0 { return false; }

    // 2*k capacity should prevent hashset from resizing
    let mut set = HashSet::with_capacity(k as usize * 2);
    let mut l = 0;

    for (r, num) in nums.iter().enumerate() {
        if set.contains(num) {
            return true;
        }

        if r - l >= k as usize {
            set.remove(&nums[l]);
            l += 1;
        }

        set.insert(num);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_nearby_duplicate_1() {
        assert!(contains_nearby_duplicate_1(vec![1, 2, 3, 1], 3));
        assert!(contains_nearby_duplicate_1(vec![1, 0, 1, 1], 1));
        assert!(!contains_nearby_duplicate_1(vec![1, 2, 3, 1, 2, 3], 2));
    }

    #[test]
    fn test_contains_nearby_duplicate_2() {
        assert!(contains_nearby_duplicate_2(vec![1, 2, 3, 1], 3));
        assert!(contains_nearby_duplicate_2(vec![1, 0, 1, 1], 1));
        assert!(!contains_nearby_duplicate_2(vec![1, 2, 3, 1, 2, 3], 2));
    }
}
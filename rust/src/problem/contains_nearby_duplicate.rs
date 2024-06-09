pub fn contains_nearby_duplicate_1(nums: Vec<i32>, k: i32) -> bool {
    let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if let Some(prev_i) = map.get_mut(num) {
            if (*prev_i - i as i32).abs() <= k {
                return true;
            } else {
                *prev_i = i as i32;
            }
        } else {
            map.insert(*num, i as i32);
        }
    }

    false
}

// Slightly better solution with two pointers + we're keeping map size <= k
pub fn contains_nearby_duplicate_2(nums: Vec<i32>, k: i32) -> bool {
    if k == 0 {
        return false;
    }

    let mut map = std::collections::HashMap::with_capacity(k as usize);

    let mut i = 0;
    let mut j = 0;

    while j < nums.len() {
        if map.contains_key(&nums[j]) {
            return true;
        }

        if j - i >= k as usize {
            map.remove(&nums[i]);
            i += 1;
        }

        map.insert(nums[j], j);
        j += 1;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_nearby_duplicate_1() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        assert!(contains_nearby_duplicate_1(nums, k));

        let nums = vec![1, 0, 1, 1];
        let k = 1;
        assert!(contains_nearby_duplicate_1(nums, k));

        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        assert!(!contains_nearby_duplicate_1(nums, k));
    }

    #[test]
    fn test_contains_nearby_duplicate_2() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        assert!(contains_nearby_duplicate_2(nums, k));

        let nums = vec![1, 0, 1, 1];
        let k = 1;
        assert!(contains_nearby_duplicate_2(nums, k));

        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        assert!(!contains_nearby_duplicate_2(nums, k));
    }
}
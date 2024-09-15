use std::collections::BTreeSet;

// https://leetcode.com/problems/contains-duplicate-iii/
fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
    let mut set = BTreeSet::new();

    for (i, &num) in nums.iter().enumerate() {
        if set.range(num - value_diff..=num + value_diff).next().is_some() {
            return true;
        }

        set.insert(num);

        if i as i32 >= index_diff {
            set.remove(&nums[i - index_diff as usize]);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0));
        assert!(contains_nearby_almost_duplicate(vec![1, 0, 1, 1], 1, 2));
        assert!(!contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3));
    }
}
// nums: [4,1,7,3,5,9]; index_diff: 2, value_diff: 2
// 1.num 4: no items in tree
//      add 4 to the tree: [4]
// 2. num 1: range is -1..=3
//      tree doesn't have any
//      add 1 to the tree: [1, 4]
// 3. num 7: range is 5..=9
//      tree doesn't have any
//      add 7 to the tree: [1, 4, 7]
//      remove 4 from the tree: [1, 7]
// 4. num 3: range is 1..=5
//      tree has 1 which is in the range
// return true

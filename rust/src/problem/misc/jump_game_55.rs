use std::collections::HashSet;

// TC - O(n^2); SC - O(n)
pub fn can_jump(nums: Vec<i32>) -> bool {
    end_reachable(&nums, &mut HashSet::<usize>::new(), 0)
}

fn end_reachable(nums: &Vec<i32>, checked: &mut HashSet<usize>, i: usize) -> bool {
    if i >= nums.len() - 1 {
        return true;
    }

    if checked.contains(&i) || nums[i] == 0 {
        return false;
    }

    checked.insert(i);

    for dist in (1..=nums[i]).rev() {
        if end_reachable(nums, checked, i + dist as usize) {
            return true;
        }
    }

    false
}

pub fn can_jump_n(nums: Vec<i32>) -> bool {
    let mut last_good_i = nums.len() - 1;

    for i in (0..nums.len() - 1).rev() {
        if i + nums[i] as usize >= last_good_i {
            last_good_i = i;
        }
    }

    last_good_i == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(can_jump(vec![2, 3, 1, 1, 4]));
        assert!(!can_jump(vec![3, 2, 1, 0, 4]));
        assert!(can_jump(vec![3, 0, 8, 2, 0, 0, 1]));
        assert!(can_jump(vec![0]));
        assert!(!can_jump(vec![1, 0, 1, 0]));
        assert!(can_jump(vec![1, 1, 1, 0]));

        assert!(can_jump_n(vec![2, 3, 1, 1, 4]));
        assert!(!can_jump_n(vec![3, 2, 1, 0, 4]));
        assert!(can_jump_n(vec![3, 0, 8, 2, 0, 0, 1]));
        assert!(can_jump_n(vec![0]));
        assert!(!can_jump_n(vec![1, 0, 1, 0]));
        assert!(can_jump_n(vec![1, 1, 1, 0]));
    }
}
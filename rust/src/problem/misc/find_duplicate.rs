// https://leetcode.com/problems/find-the-duplicate-number
// Uses Floyd's slow and fast pointers.
// Probably the only O(n) solution that complies problem requirements.

// Since numbers are in the range [1, n] and array len is n + 1, each entry is a valid index:
// nums[nums[i]] is valid for any i in range [0, n + 1];
// Thus, we can treat the array as a linked list with a cycle (since it has a duplicate entry)
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut s = nums[0];
    let mut f = nums[0];

    loop {
        // Phase 1: move slow one step and fast two steps at a time;
        s = nums[s as usize];
        f = nums[nums[f as usize] as usize];

        // Once they met, both of them are trapped in the cycle;
        if s == f {
            // Reset slow pointer;
            s = nums[0];
            // Phase 2: move them with the same speed until they meet;
            while s != f {
                s = nums[s as usize];
                f = nums[f as usize];
            }

            return s;
        }
    }
}

// This solution doesn't meet the problem constraints because it mutates the input;
fn find_duplicate_sort(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();

    for w in nums.windows(2) {
        if w[0] == w[1] {
            return w[0];
        }
    }

    unreachable!()
}

// This solution mutates input as well as the preious one hence doesn't comply with constraints;
// [1233] -> [2133] -> [3123] -> 3
fn find_duplicate_swap(mut nums: Vec<i32>) -> i32 {
    while nums[0] != nums[nums[0] as usize] {
        let b = nums[0] as usize;
        nums.swap(0, b)
    }

    nums[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate() {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(find_duplicate(nums), 2);

        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(find_duplicate(nums), 3);

        let nums = vec![1, 1];
        assert_eq!(find_duplicate(nums), 1);

        let nums = vec![1, 1, 2];
        assert_eq!(find_duplicate(nums), 1);

        let nums = vec![3, 3, 3, 3, 3];
        assert_eq!(find_duplicate(nums), 3);

        let nums = vec![4, 3, 1, 4, 2];
        assert_eq!(find_duplicate(nums), 4);
    }

    #[test]
    fn test_find_duplicate_sort() {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(find_duplicate_sort(nums), 2);
    }

    #[test]
    fn test_find_duplicate_swap() {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(find_duplicate_swap(nums), 2);
    }
}
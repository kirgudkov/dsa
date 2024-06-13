// https://leetcode.com/problems/find-the-duplicate-number
// Freaking incomprehensible approach. TC is O(n) and SC is O(1).
// Since numbers are in the range [1, n] and array len is n + 1, every number is a valid index.
// So we can treat the array as a linked list and use fast and slow pointers;
// First, we move slow pointer one step at the time and fast pointer two steps at the time until they meet.
// Then we reset slow pointer back to the start and move both pointer with the same speed.
// Once pointers become equal we've found the answer.
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut s = nums[0];
    let mut f = nums[0];

    loop {
        s = nums[s as usize];
        f = nums[nums[f as usize] as usize];

        if s == f {
            s = nums[0];

            while s != f {
                s = nums[s as usize];
                f = nums[f as usize];
            }

            return s;
        }
    }
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
}
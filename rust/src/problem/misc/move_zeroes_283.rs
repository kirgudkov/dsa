use crate::utils::partition_in_place;

// https://leetcode.com/problems/move-zeroes
// The problem just boils down to the in place partitioning
pub fn move_zeroes(nums: &mut [i32]) {
    partition_in_place(nums, |&i| i != 0);
}

pub fn move_zeroes_1(nums: &mut [i32]) {
    let mut p1 = 0;
    let mut p2 = 0;

    while p1 < nums.len() && p2 < nums.len() {
        nums.swap(p1, p2);

        while p1 < nums.len() && nums[p1] != 0 { p1 += 1; }
        // p1 is on zero now
        p2 = p1;
        // starting from p1, looking for the closest non-zero element
        while p2 < nums.len() && nums[p2] == 0 { p2 += 1; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes_1(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }
}

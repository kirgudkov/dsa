use crate::utils::partition_in_place;

// https://leetcode.com/problems/move-zeroes
// The problem just boils down to the in place partitioning
pub fn move_zeroes(nums: &mut [i32]) {
    partition_in_place(nums, |&i| i != 0);
}

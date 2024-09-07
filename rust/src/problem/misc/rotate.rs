pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let pos = nums.len() - (k as usize % nums.len());
    nums.clone_from(&[&nums[pos..], &nums[0..pos]].concat());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);

        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 10);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }
}
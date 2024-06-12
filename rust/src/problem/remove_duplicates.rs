pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
    let mut p = 0;

    for i in 1..nums.len() {
        if nums[i] != nums[p] {
            p += 1;
            nums[p] = nums[i];
        }
    }

    p as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(remove_duplicates(&mut nums), 2);
        assert_eq!(nums, vec![1, 2, 2]);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4]);

        let mut nums = vec![3];
        assert_eq!(remove_duplicates(&mut nums), 1);
        assert_eq!(nums, vec![3]);
    }
}
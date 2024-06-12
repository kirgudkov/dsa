pub fn move_zeroes(nums: &mut [i32]) {
    let mut p = 0;
    
    while p < nums.len() && nums[p] != 0 {
        p += 1;
    }

    for i in p + 1..nums.len() {
        if nums[i] != 0 && nums[p] == 0 {
            nums.swap(i, p);
            p += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        let mut nums = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);

        let mut nums = vec![1];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1]);

        let mut nums = vec![1, 0, 1];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 1, 0])
    }
}
pub fn longest_ones(nums: Vec<i32>, mut k: i32) -> i32 {
    let mut l = 0;
    let mut r = 0;

    while r < nums.len() {
        if nums[r] == 0 {
            k -= 1;
        }


        if k < 0 {
            if nums[l] == 0 {
                k += 1;
            }
            l += 1;
        }

        r += 1;
    }

    (r - l) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6);
        assert_eq!(longest_ones(vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], 3), 10);
        assert_eq!(longest_ones(vec![0, 0, 1, 1, 1, 0, 0], 0), 3);
        assert_eq!(longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1], 0), 4);
    }
}

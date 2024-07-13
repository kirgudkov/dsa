pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return 0;
    }

    nums.sort_unstable();

    let mut count = 0;
    let mut i = 0;

    while i < nums.len() - 2 {
        let mut j = i + 1;
        let mut k = i + 2;

        while j < nums.len() - 1 && nums[i] != 0 {
            while k < nums.len() && nums[i] + nums[j] > nums[k] {
                k += 1;
            }
            count += k - j - 1;
            j += 1;
        }

        i += 1;
    }

    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(triangle_number(vec![2, 2, 3, 4]), 3);
        assert_eq!(triangle_number(vec![4, 2, 3, 4]), 4);
    }
}
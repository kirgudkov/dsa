pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    let mut l = 1;
    let mut r = *nums.iter().max().unwrap();

    while l < r {
        let m = l + (r - l) / 2;

        if nums.iter().map(|&x| (x as f64 / m as f64).ceil()).sum::<f64>() > threshold as f64 {
            l = m + 1;
        } else {
            r = m;
        }
    }

    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 5, 9];
        let threshold = 6;
        assert_eq!(smallest_divisor(nums, threshold), 5);

        let nums = vec![2, 3, 5, 7, 11];
        let threshold = 11;
        assert_eq!(smallest_divisor(nums, threshold), 3);

        let nums = vec![19];
        let threshold = 5;
        assert_eq!(smallest_divisor(nums, threshold), 4);
    }
}
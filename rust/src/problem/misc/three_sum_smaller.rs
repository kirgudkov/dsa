pub fn three_sum_smaller(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();
    let mut count = 0;

    for i in 0..nums.len() {
        let mut l = i + 1;
        let mut r = nums.len() - 1;

        while l < r {
            if nums[i] + nums[l] + nums[r] < target {
                count += r - l;
                l += 1;
            } else {
                r -= 1;
            }
        }
    }

    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(three_sum_smaller(vec![-2, 0, 1, 3], 2), 2);
        assert_eq!(three_sum_smaller(vec![3, 1, 0, -2], 4), 3);
        assert_eq!(three_sum_smaller(vec![0, 0, 0], 0), 0);
    }
}
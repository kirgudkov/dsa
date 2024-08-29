use crate::sort::radix_sort::radix_sort;

// https://leetcode.com/problems/maximum-gap
fn maximum_gap(mut nums: Vec<i32>) -> i32 {
    radix_sort(&mut nums);

    nums.windows(2)
        .map(|w| w[1] - w[0])
        .max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::problem::maximum_gap::maximum_gap;

    #[test]
    fn test_maximum_gap() {
        assert_eq!(maximum_gap(vec![3, 6, 9, 1]), 3);
        assert_eq!(maximum_gap(vec![10]), 0);
    }
}

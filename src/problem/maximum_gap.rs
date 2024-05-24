use crate::problem::radix_sort::radix_sort;

pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
    radix_sort(&mut nums);

    nums.windows(2)
        .map(|x| x[1] - x[0])
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_gap() {
        assert_eq!(maximum_gap(vec![3, 6, 9, 1]), 3);
        assert_eq!(maximum_gap(vec![10]), 0);
    }
}
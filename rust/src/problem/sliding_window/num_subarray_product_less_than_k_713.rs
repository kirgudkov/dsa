pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    if k <= 1 {
        return 0;
    }

    let mut res = 0;

    let mut l = 0;
    let mut p = 1;

    for (r, num) in nums.iter().enumerate() {
        p *= num;

        while p >= k {
            p /= nums[l];
            l += 1;
        }

        res += r - l + 1;
    }

    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100), 8);
        assert_eq!(num_subarray_product_less_than_k(vec![1, 2, 3], 0), 0);
        assert_eq!(num_subarray_product_less_than_k(vec![1, 1, 1], 1), 0);
    }
}
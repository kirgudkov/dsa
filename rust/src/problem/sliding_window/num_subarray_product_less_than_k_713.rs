// https://leetcode.com/problems/subarray-product-less-than-k
pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    if k <= 1 {
        return 0;
    }

    let mut count = 0;
    let mut product = 1;
    let mut l = 0;

    for (r, num) in nums.iter().enumerate() {
        product *= num;

        while product >= k {
            product /= nums[l];
            l += 1;
        }

        /* This is the most trickiest and essential part. Not intuitive at all.
        Even though r - l + 1 represents the size of the window, 
        it is also give us the number of subarrays that **end with r-th element** (not all possible subarrays!):
        for [1,2,3] it gives us 3 subarrays: [1,2,3], [2,3], [3]; The immediate question would be: why did we ignore [1], [2], [1,2]?
        The answer is - we didn't. They were counted in a cumulative way while we were moving our right pointer before we had reached 3th;
        */
        count += r - l + 1;
    }

    count as i32
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
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut product_except_self = vec![1; nums.len()];

    let mut prefix = vec![1; nums.len() + 1];
    let mut postfix = vec![1; nums.len() + 1];

    let (mut p1, mut p2) = (1, 1);

    for (i, &num) in nums.iter().enumerate() {
        p1 *= num;
        prefix[i + 1] = p1;

        p2 *= nums[nums.len() - i - 1];
        postfix[i + 1] = p2;
    }

    for i in 0..nums.len() {
        product_except_self[i] = prefix[i] * postfix[nums.len() - i - 1];
    }

    product_except_self
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 4];
        let res = vec![24, 12, 8, 6];
        assert_eq!(product_except_self(nums), res);
    }
}
pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
    let mut prefix = vec![0; nums.len()];

    let mut sum = 0;
    for (i, &num) in nums.iter().enumerate() {
        sum += num;
        prefix[i] = sum;
    }

    let mut longest = 0;
    let mut map = std::collections::HashMap::new();

    for (i, &sum) in prefix.iter().enumerate() {
        if sum == k {
            longest = i + 1;
        } else if let Some(&j) = map.get(&(sum - k)) {
            longest = longest.max(i - j);
        }

        map.entry(sum).or_insert(i);
    }

    longest as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(max_sub_array_len(vec![1, 1, 0], 1), 2);
        assert_eq!(max_sub_array_len(vec![1, 0, -1], -1), 2);
        assert_eq!(max_sub_array_len(vec![-1, 1], 1), 1);
        assert_eq!(max_sub_array_len(vec![1, -1, 5, -2, 3], 3), 4);
    }
}

fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();

    let mut subarrays = vec![];

    fn backtrack(nums: &[i32], subarrays: &mut Vec<Vec<i32>>, buf: &mut Vec<i32>, j: usize) {
        subarrays.push(buf.clone());

        for i in j..nums.len() {
            if i != j && nums[i - 1] == nums[i] {
                continue;
            }

            buf.push(nums[i]);
            backtrack(nums, subarrays, buf, i + 1);
            buf.pop();
        }
    }

    backtrack(&nums, &mut subarrays, &mut Vec::new(), 0);
    subarrays
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 2];
        let res = vec![vec![], vec![1], vec![1, 2], vec![1, 2, 2], vec![2], vec![2, 2]];
        assert_eq!(subsets_with_dup(nums), res);
    }
}
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sets = vec![];

    fn backtrack(nums: &[i32], sets: &mut Vec<Vec<i32>>, buf: &mut Vec<i32>, pos: usize) {
        sets.push(buf.clone());

        for i in pos..nums.len() {
            buf.push(nums[i]);
            backtrack(nums, sets, buf, i + 1);
            buf.pop();
        }
    }

    backtrack(&nums, &mut sets, &mut Vec::new(), 0);
    sets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3];
        let mut res = subsets(nums);
        res.sort();
        let mut expected = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 3],
            vec![2],
            vec![2, 3],
            vec![3],
        ];
        expected.sort();
        assert_eq!(res, expected);
    }
}
pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort_unstable();
    candidates.retain(|e| *e <= target);

    let mut result = vec![];

    fn backtrack(
        candidates: &[i32],
        target: i32,
        result: &mut Vec<Vec<i32>>,
        buf: &mut Vec<i32>,
        sum: i32,
        j: usize,
    ) {
        if sum == target {
            result.push(buf.clone());
            return;
        }

        for i in j..candidates.len() {
            if sum + candidates[i] > target || (i != j && candidates[i] == candidates[i - 1]) {
                continue;
            }

            buf.push(candidates[i]);
            backtrack(candidates, target, result, buf, sum + candidates[i], i + 1);
            buf.pop();
        }
    }

    backtrack(&candidates, target, &mut result, &mut vec![], 0, 0);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let result = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
        assert_eq!(combination_sum2(candidates, target), result);
    }
}
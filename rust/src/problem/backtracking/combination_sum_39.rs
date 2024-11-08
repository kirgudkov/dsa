pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort_unstable();
    // cut everything that greater than target
    candidates.retain(|&x| x <= target);

    let mut result = vec![];

    fn backtrack(candidates: &Vec<i32>, target: i32, result: &mut Vec<Vec<i32>>, buf: &mut Vec<i32>, sum: i32, j: usize) {
        if sum == target {
            result.push(buf.clone());
            return;
        }

        for i in j..candidates.len() {
            if sum + candidates[i] > target {
                continue;
            }

            buf.push(candidates[i]);
            backtrack(candidates, target, result, buf, sum + candidates[i], i);
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
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let res = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(combination_sum(candidates, target), res);
    }
}
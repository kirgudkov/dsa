fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];

    fn backtrack(
        result: &mut Vec<Vec<i32>>,
        buf: &mut Vec<i32>,
        current_sum: i32,
        target_sum: i32,
        k: usize, // candidates upper bound (end index)
        j: usize // current start index
    ) {
        if current_sum == target_sum && buf.len() == k {
            result.push(buf.clone());
            return;
        }

        for i in j..=9 {
            if current_sum + i as i32 > target_sum {
                continue;
            }

            buf.push(i as i32);
            backtrack(result, buf, current_sum + i as i32, target_sum, k, i + 1);
            buf.pop();
        }
    }

    backtrack(&mut result, &mut vec![], 0, n, k as usize, 1);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let k = 3;
        let n = 7;
        let result = combination_sum3(k, n);
        assert_eq!(result, vec![vec![1, 2, 4]]);
    }
}
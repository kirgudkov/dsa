// Finds all k-length combinations of numbers from 1 to n
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];

    fn backtrack(start: usize, k: usize, n: usize, buf: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if buf.len() == k {
            return res.push(buf.clone());
        }

        for num in start..=n {
            buf.push(num as i32);
            backtrack(num + 1, k, n, buf, res);
            buf.pop();
        }
    }

    backtrack(1, k as usize, n as usize, &mut vec![], &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(combine(4, 2), vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 4]]);
        assert_eq!(combine(4, 3), vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 3, 4], vec![2, 3, 4]]);
    }
}

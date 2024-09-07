pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    let mut i = 0;

    while i != num_rows as usize {
        let mut row = vec![];
        let mut j = 0;

        while j <= i {
            if j != 0 && j != i {
                row.push(res[i - 1][j - 1] + res[i - 1][j]);
            } else {
                row.push(1);
            }

            j += 1;
        }

        res.push(row);
        i += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        assert_eq!(generate(5), vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]]);
        assert_eq!(generate(1), vec![vec![1]]);
    }
}

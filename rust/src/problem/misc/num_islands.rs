pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '1' {
                count += 1;
                flood(&mut grid, i, j);
            }
        }
    }

    count
}

fn flood(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
    if i >= grid.len() || j >= grid[i].len() || grid[i][j] == '0' {
        return;
    }

    grid[i][j] = '0';

    flood(grid, i, j + 1);
    flood(grid, i + 1, j);
    flood(grid, i, j.wrapping_sub(1));
    flood(grid, i.wrapping_sub(1), j);
}

#[cfg(test)]
mod tests {
    use crate::problem::misc::num_islands::num_islands;

    #[test]
    fn test_num_islands() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(num_islands(grid), 3);

        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];

        assert_eq!(num_islands(grid), 1);

        let grid = vec![
            vec!['1', '1', '1'],
            vec!['0', '1', '0'],
            vec!['1', '1', '1'],
        ];

        assert_eq!(num_islands(grid), 1);

        let grid = vec![
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1'],
            vec!['1', '1', '1', '0', '1'],
        ];

        assert_eq!(num_islands(grid), 1);
    }
}

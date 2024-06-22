// https://leetcode.com/problems/rotting-oranges
// TC: O(mn), SC: O(mn)
pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut q = std::collections::VecDeque::new();
    let mut max = 0;

    // Push all rotten oranges to the queue
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            // It takes 0 minutes for a rotten orange to rot
            if grid[i][j] == 2 { q.push_back((i, j, 0)) }
        }
    }

    fn neighbors(i: usize, j: usize, grid: &[Vec<i32>]) -> Vec<(usize, usize)> {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|&(di, dj)| {
                (i as isize + di, j as isize + dj)
            })
            .filter(|&(i, j)| {
                i >= 0 && i < grid.len() as isize && j >= 0 && j < grid[0].len() as isize
            })
            .map(|(i, j)| {
                (i as usize, j as usize)
            })
            .collect()
    }

    while let Some((i, j, d)) = q.pop_front() {
        max = max.max(d);

        for (i, j) in neighbors(i, j, &grid) {
            if grid[i][j] == 1 {
                grid[i][j] = 2;
                q.push_back((i, j, d + 1));
            }
        }
    }

    for row in &grid {
        if row.contains(&1) { return -1; }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oranges_rotting() {
        let grid = vec![
            vec![2, 1, 1],
            vec![1, 1, 0],
            vec![0, 1, 1],
        ];
        assert_eq!(oranges_rotting(grid), 4);


        let grid = vec![
            vec![0, 2],
        ];
        assert_eq!(oranges_rotting(grid), 0);

        let grid = vec![
            vec![2, 1, 1],
            vec![0, 1, 1],
            vec![1, 0, 1],
        ];
        assert_eq!(oranges_rotting(grid), -1);
    }
}

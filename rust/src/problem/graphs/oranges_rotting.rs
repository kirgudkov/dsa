use crate::utils::Neighbors;

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

    while let Some((i, j, d)) = q.pop_front() {
        max = max.max(d);

        for (i, j) in grid.neighbors(i, j) {
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

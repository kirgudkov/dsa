use crate::utils::Neighbors;
use std::collections::VecDeque;

// https://leetcode.com/problems/rotting-oranges
// TC: O(mn), SC: O(mn)
pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    // Initialize a queue with all rotten oranges
    let mut queue = grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &val)| {
                (val == 2).then_some((i, j, 0))
            })
        }).collect::<VecDeque<_>>();

    let mut elapsed = 0;

    while let Some((i, j, minutes)) = queue.pop_front() {
        elapsed = elapsed.max(minutes);

        grid.neighbors(i, j).iter().for_each(|&(i, j)| {
            if grid[i][j] == 1 {
                grid[i][j] = 2;
                queue.push_back((i, j, minutes + 1));
            }
        });
    }

    for row in &grid {
        if row.contains(&1) { return -1; }
    }

    elapsed
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

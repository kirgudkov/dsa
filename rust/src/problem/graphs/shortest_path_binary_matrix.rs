use std::collections::VecDeque;

pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
    if grid[0][0] != 0 || grid[grid.len() - 1][grid[0].len() - 1] != 0 {
        return -1;
    }

    let mut queue = VecDeque::from([(0, 0)]);
    grid[0][0] = 1;

    while let Some((i, j)) = queue.pop_front() {
        if i == grid.len() - 1 && j == grid[i].len() - 1 {
            return grid[i][j];
        }

        for (di, dj) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if ni >= 0 && ni < grid.len() as i32 && nj >= 0 && nj < grid[0].len() as i32 && grid[ni as usize][nj as usize] == 0 {
                grid[ni as usize][nj as usize] = grid[i][j] + 1;
                queue.push_back((ni as usize, nj as usize));
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path_binary_matrix() {
        let grid = vec![
            vec![0, 1],
            vec![1, 0],
        ];
        assert_eq!(shortest_path_binary_matrix(grid), 2);

        let grid = vec![
            vec![0, 0, 0],
            vec![1, 1, 0],
            vec![1, 1, 0],
        ];
        assert_eq!(shortest_path_binary_matrix(grid), 4);

        let grid = vec![
            vec![1, 0, 0],
            vec![1, 1, 0],
            vec![1, 1, 0],
        ];
        assert_eq!(shortest_path_binary_matrix(grid), -1);

        let grid = vec![
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 0],
        ];
        assert_eq!(shortest_path_binary_matrix(grid), 4);
    }
}

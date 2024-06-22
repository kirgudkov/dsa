pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
    if grid[0][0] != 0 || grid[grid.len() - 1][grid[0].len() - 1] != 0 {
        return -1;
    }

    let mut q = std::collections::VecDeque::from([(0, 0)]);
    grid[0][0] = 1;

    while let Some((i, j)) = q.pop_front() {
        if i == grid.len() - 1 && j == grid[i].len() - 1 {
            return grid[i][j];
        }

        for (del_i, del_j) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
            let new_i = i as i32 + del_i;
            let new_j = j as i32 + del_j;

            if new_i >= 0 && new_i < grid.len() as i32 && new_j >= 0 && new_j < grid[0].len() as i32 && grid[new_i as usize][new_j as usize] == 0 {
                grid[new_i as usize][new_j as usize] = grid[i][j] + 1;
                q.push_back((new_i as usize, new_j as usize));
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

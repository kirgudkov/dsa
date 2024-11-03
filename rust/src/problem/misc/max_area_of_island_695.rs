// TC - O(m*n)
pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut max_area = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                max_area = max_area.max(calc_area(&mut grid, i, j));
            }
        }
    }

    max_area
}

// SC - O(m*n)
fn calc_area(grid: &mut [Vec<i32>], i: usize, j: usize) -> i32 {
    let mut area = 0;
    let mut stack = vec![(i, j)];
    let mut visited = std::collections::HashSet::new();

    while let Some((i, j)) = stack.pop() {
        if visited.contains(&(i, j)) {
            continue;
        }

        visited.insert((i, j));
        grid[i][j] = 0;
        area += 1;

        for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let i = i as isize + di;
            let j = j as isize + dj;

            let i_in_bounds = i >= 0 && i < grid.len() as isize;
            let j_in_bounds = j >= 0 && j < grid[0].len() as isize;

            if i_in_bounds && j_in_bounds && grid[i as usize][j as usize] == 1 {
                stack.push((i as usize, j as usize));
            }
        }
    }

    area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 1],
        ];

        assert_eq!(max_area_of_island(grid), 4);

        let grid = vec![
            vec![1, 1, 1, 1, 0],
            vec![1, 1, 0, 1, 0],
            vec![1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];

        assert_eq!(max_area_of_island(grid), 9);

        let grid = vec![
            vec![1, 1, 1],
            vec![0, 1, 0],
            vec![1, 1, 1],
        ];

        assert_eq!(max_area_of_island(grid), 7);
    }
}
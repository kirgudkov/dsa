fn part_1(input: &str) -> i32 {
    let mut grid = input.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    let (mut i, mut j) = guard_pos(&grid);

    loop {
        if walked_out(&mut grid, &mut i, &mut j) {
            break;
        }
    }

    // 1 is added to the result because the guard is not counted
    grid.iter().flatten().filter(|&&c| c == 'X').count() as i32 + 1
}

// That's f*in cursed, runs 1 min on M2 Max 🤭
fn part_2(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (i, j) = guard_pos(&grid);

    let simulate = |mut grid: Vec<Vec<char>>| -> bool {
        let (mut i, mut j) = (i, j);
        let mut visited = std::collections::HashSet::new();

        loop {
            // It's important to match initial guard direction
            if !visited.insert((i, j, grid[i][j])) {
                return true;
            }

            if walked_out(&mut grid, &mut i, &mut j) {
                break;
            }
        }

        false
    };

    let mut empty_cells = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '.' {
                empty_cells.push((i, j));
            }
        }
    }

    let mut count = 0;
    for (i, j) in empty_cells {
        let mut grid = grid.clone();
        grid[i][j] = '#';
        if simulate(grid) {
            count += 1;
        }
    }

    count
}

fn walked_out(grid: &mut [Vec<char>], i: &mut usize, j: &mut usize) -> bool {
    match grid[*i][*j] {
        '^' => {
            if *i == 0 {
                return true;
            }

            if grid[*i - 1][*j] != '#' {
                grid[*i][*j] = 'X';
                *i -= 1;
                grid[*i][*j] = '^';
            } else {
                grid[*i][*j] = '>';
            }
        }
        'v' => {
            if *i == grid.len() - 1 {
                return true;
            }

            if grid[*i + 1][*j] != '#' {
                grid[*i][*j] = 'X';
                *i += 1;
                grid[*i][*j] = 'v';
            } else {
                grid[*i][*j] = '<';
            }
        }
        '<' => {
            if *j == 0 {
                return true;
            }

            if grid[*i][*j - 1] != '#' {
                grid[*i][*j] = 'X';
                *j -= 1;
                grid[*i][*j] = '<';
            } else {
                grid[*i][*j] = '^';
            }
        }
        '>' => {
            if *j == grid[0].len() - 1 {
                return true;
            }

            if grid[*i][*j + 1] != '#' {
                grid[*i][*j] = 'X';
                *j += 1;
                grid[*i][*j] = '>';
            } else {
                grid[*i][*j] = 'v';
            }
        }
        _ => return true,
    }

    false
}

fn guard_pos(grid: &[Vec<char>]) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if matches!(grid[i][j], '^' | '>' | '<' | 'v') {
                return (i, j);
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(part_1(input), 41);
        dbg!(part_1(&std::fs::read_to_string("./src/aoc/inputs/day_6").unwrap()));
        assert_eq!(part_2(input), 6);
        // dbg!(part_2(&std::fs::read_to_string("./src/aoc/inputs/day_6").unwrap()));
    }
}

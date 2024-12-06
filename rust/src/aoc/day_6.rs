fn part_1(input: &str) -> i32 {
    let mut grid = input
        .lines()
        .map(|line| line.split("").flat_map(|word| word.chars()))
        .map(|chars| chars.collect())
        .collect::<Vec<Vec<char>>>();

    let (mut i, mut j) = (0, 0);

    for _i in 0..grid.len() {
        for _j in 0..grid[0].len() {
            if matches!(grid[_i][_j], '^' | '>' | '<' | 'v') {
                (i, j) = (_i, _j);
            }
        }
    }

    loop {
        match grid[i][j] {
            '^' => {
                if i == 0 {
                    break;
                }

                if grid[i - 1][j] != '#' {
                    grid[i][j] = 'X';
                    i -= 1;
                    grid[i][j] = '^';
                } else {
                    grid[i][j] = '>';
                }
            }
            'v' => {
                if i == grid.len() - 1 {
                    break;
                }

                if grid[i + 1][j] != '#' {
                    grid[i][j] = 'X';
                    i += 1;
                    grid[i][j] = 'v';
                } else {
                    grid[i][j] = '<';
                }
            }
            '<' => {
                if j == 0 {
                    break;
                }

                if grid[i][j - 1] != '#' {
                    grid[i][j] = 'X';
                    j -= 1;
                    grid[i][j] = '<';
                } else {
                    grid[i][j] = '^';
                }
            }
            '>' => {
                if j == grid[0].len() - 1 {
                    break;
                }

                if grid[i][j + 1] != '#' {
                    grid[i][j] = 'X';
                    j += 1;
                    grid[i][j] = '>';
                } else {
                    grid[i][j] = 'v';
                }
            }
            _ => panic!("wtf where's the guard?"),
        }
    }

    // 1 is added to the result because the guard is not counted
    grid.iter().flatten().filter(|&&c| c == 'X').count() as i32 + 1
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
    }
}

use crate::utils::InBounds;

fn part_1(input: &str, target: &[char]) -> i32 {
    let mut input = into_vec(input);
    let mut result = 0;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == target[0] {
                for direction in Direction::ALL {
                    let delta = direction.delta();
                    // This is an almost classic "word search" backtracking problem. The only difference is that we should preserve direction in all
                    // subsequent backtrack calls i.e word can't have zig-zag shape. We pass initial delta and change indices according to this delta
                    if backtrack(&mut input, target, 1, i as i32 + delta.0, j as i32 + delta.1, delta) {
                        result += 1;
                    }
                }
            }
        }
    }

    result
}

fn part_2(input: &str, target: &[char]) -> i32 {
    let m = target.len() / 2;
    let target_rev = target.iter().rev().cloned().collect::<Vec<char>>();
    let mut input = into_vec(input);
    let mut result = 0;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == target[m] {
                // That's a pretty much cursed approach. Every time it meets target's mid char, it expands in four diagonal directions
                // and checks if it can form a half of the target.
                let delta = Direction::UpLeft.delta();
                let a = backtrack(&mut input, &target[m..], 1, i as i32 + delta.0, j as i32 + delta.1, delta);
                let delta = Direction::DownRight.delta();
                let b = backtrack(&mut input, &target_rev[m..], 1, i as i32 + delta.0, j as i32 + delta.1, delta);
                let diag_1 = a && b;

                let delta = Direction::UpLeft.delta();
                let a = backtrack(&mut input, &target_rev[m..], 1, i as i32 + delta.0, j as i32 + delta.1, delta);
                let delta = Direction::DownRight.delta();
                let b = backtrack(&mut input, &target[m..], 1, i as i32 + delta.0, j as i32 + delta.1, delta);
                let diag_1_rev = a && b;

                let delta = Direction::UpRight.delta();
                let a = backtrack(&mut input, &target[m..], 1, i as i32 + delta.0, j as i32 + delta.1, delta);
                let delta = Direction::DownLeft.delta();
                let b = backtrack(&mut input, &target_rev[m..], 1, i as i32 + delta.0, j as i32 + delta.1, delta);
                let diag_2 = a && b;

                let delta = Direction::UpRight.delta();
                let a = backtrack(&mut input, &target_rev[m..], 1, i as i32 + delta.0, j as i32 + delta.1, delta);
                let delta = Direction::DownLeft.delta();
                let b = backtrack(&mut input, &target[m..], 1, i as i32 + delta.0, j as i32 + delta.1, delta);
                let diag_2_rev = a && b;

                if (diag_1 || diag_1_rev) && (diag_2 || diag_2_rev) {
                    result += 1;
                }
            }
        }
    }

    result
}

fn into_vec(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.split_whitespace().flat_map(|word| word.chars()))
        .map(|chars| chars.collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn backtrack(input: &mut [Vec<char>], target: &[char], k: usize, i: i32, j: i32, delta: (i32, i32)) -> bool {
    if k == target.len() {
        return true;
    }

    if !input.in_bounds(i, j) || input[i as usize][j as usize] != target[k] {
        return false;
    }

    let original = input[i as usize][j as usize];
    input[i as usize][j as usize] = '.';

    let found = backtrack(input, target, k + 1, i + delta.0, j + delta.1, delta);
    input[i as usize][j as usize] = original;

    found
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    const ALL: [Direction; 8] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::UpLeft,
        Direction::UpRight,
        Direction::DownLeft,
        Direction::DownRight,
    ];

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (-1, 1),
            Direction::DownLeft => (1, -1),
            Direction::DownRight => (1, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let target = &['X', 'M', 'A', 'S'];
        assert_eq!(part_1(input, target), 18);
        dbg!(part_1(&std::fs::read_to_string("./src/aoc/inputs/day_4").unwrap(), target));

        let target = &['M', 'A', 'S'];
        assert_eq!(part_2(input, target), 9);
        dbg!(part_2(&std::fs::read_to_string("./src/aoc/inputs/day_4").unwrap(), target));
    }
}

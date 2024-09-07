use std::collections::VecDeque;

// https://leetcode.com/problems/open-the-lock
// BFS approach
pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    if target == "0000" {
        return 0;
    }

    let target = target.parse::<usize>().unwrap();

    let mut q = VecDeque::from(vec![0]);
    let mut visited = vec![false; 10000];
    let mut depth = 0;

    // We initially prevent blacklisted codes from visiting:
    deadends.iter().for_each(|item| {
        visited[item.parse::<usize>().unwrap()] = true;
    });

    while !q.is_empty() {
        for _ in 0..q.len() {
            let code = q.pop_front().unwrap();

            if visited[code] {
                continue;
            }

            visited[code] = true;

            if code == target {
                return depth;
            }

            // We could probably optimize space complexity by skipping visited options;
            q.extend(get_options(code));
        }

        depth += 1;
    }

    -1
}

// Finds all possible ways to turn knobs;
// For example, for code 0000 we could turn each knob up or down and get:
// [0001, 0009, 0010, 0090, 0100, 0900, 1000, 9000]
fn get_options(code: usize) -> Vec<usize> {
    let mut options = Vec::new();

    for position in [1, 10, 100, 1000].iter() {
        match code / position % 10 {
            0 => {
                options.push(code + position);
                options.push(code + 9 * position);
            }
            9 => {
                options.push(code - position);
                options.push(code - 9 * position);
            }
            _ => {
                options.push(code - position);
                options.push(code + position);
            }
        }
    }

    options
}

#[cfg(test)]
mod tests {
    use crate::problem::misc::open_lock_752::open_lock;

    #[test]
    fn test_open_lock() {
        let deadends = vec!["0201".to_string(), "0101".to_string(), "0102".to_string(), "1212".to_string(), "2002".to_string()];
        let target = "0202".to_string();
        assert_eq!(open_lock(deadends, target), 6);

        let deadends = vec!["8888".to_string()];
        let target = "0009".to_string();
        assert_eq!(open_lock(deadends, target), 1);

        let deadends = vec!["8887".to_string(), "8889".to_string(), "8878".to_string(), "8898".to_string(), "8788".to_string(), "8988".to_string(), "7888".to_string(), "9888".to_string()];
        let target = "8888".to_string();
        assert_eq!(open_lock(deadends, target), -1);

        let deadends = vec!["0000".to_string()];
        let target = "8888".to_string();
        assert_eq!(open_lock(deadends, target), -1);
    }
}

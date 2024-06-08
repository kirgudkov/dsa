use std::collections::VecDeque;

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    if target == "0000" {
        return 0;
    }

    let target = target.parse::<usize>().unwrap();

    let mut queue = VecDeque::from(vec![0]);
    let mut visited = vec![false; 10000];
    let mut moves = 0;

    deadends.iter().for_each(|item| {
        visited[item.parse::<usize>().unwrap()] = true;
    });

    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let num = queue.pop_front().unwrap();

            if visited[num] {
                continue;
            }

            if num == target {
                return moves;
            }

            queue.extend(get_options(num));
            visited[num] = true;
        }
        moves += 1;
    }

    -1
}

fn get_options(target: usize) -> Vec<usize> {
    let mut result = Vec::new();

    for digit_positions in [1, 10, 100, 1000].iter() {
        match target / (digit_positions) % 10 {
            0 => {
                result.push(target + digit_positions);
                result.push(target + 9 * digit_positions);
            }
            9 => {
                result.push(target - digit_positions);
                result.push(target - 9 * digit_positions);
            }
            _ => {
                result.push(target - digit_positions);
                result.push(target + digit_positions);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::problem::open_lock::open_lock;

    #[test]
    fn test_open_lock() {
        {
            let deadends = vec!["0201".to_string(), "0101".to_string(), "0102".to_string(), "1212".to_string(), "2002".to_string()];
            let target = "0202".to_string();
            let res = open_lock(deadends, target);
            assert_eq!(res, 6);
        }

        {
            let deadends = vec!["8888".to_string()];
            let target = "0009".to_string();
            let res = open_lock(deadends, target);
            assert_eq!(res, 1);
        }

        {
            let deadends = vec!["8887".to_string(), "8889".to_string(), "8878".to_string(), "8898".to_string(), "8788".to_string(), "8988".to_string(), "7888".to_string(), "9888".to_string()];
            let target = "8888".to_string();
            let res = open_lock(deadends, target);
            assert_eq!(res, -1);
        }

        {
            let deadends = vec!["0000".to_string()];
            let target = "8888".to_string();
            let res = open_lock(deadends, target);
            assert_eq!(res, -1);
        }
    }
}

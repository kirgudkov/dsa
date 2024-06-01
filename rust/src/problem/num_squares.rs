use std::collections::VecDeque;

// BFS approach
pub fn num_squares_bfs(n: i32) -> i32 {
    let mut q = VecDeque::from([n]);
    let mut moves = 0;

    while !q.is_empty() {
        moves += 1;

        for _ in 0..q.len() {
            let num = q.pop_front().unwrap();

            let mut i = 1;
            while i * i <= num {
                let next = num - i * i;
                if next == 0 {
                    return moves;
                }
                q.push_back(next);
                i += 1;
            }
        }
    }

    -1
}

// DP approach
pub fn num_squares_dp(n: i32) -> i32 {
    let mut dp = vec![0; (n + 1) as usize];
    dp[1] = 1;

    for i in 2..=n as usize {
        let (mut min, mut j) = (i, 2);
        while j * j <= i {
            min = min.min(dp[i - j * j] + 1);
            j += 1;
        }
        dp[i] = min;
    }

    dp[n as usize] as i32
}

// Recursive approach
pub fn num_squares_rec(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let mut result = n;

    for i in 1..=n {
        if i * i > n {
            break;
        } else {
            result = result.min(num_squares_rec(n - i * i) + 1);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::problem::num_squares::{num_squares_bfs, num_squares_dp, num_squares_rec};

    #[test]
    fn test_num_squares_dp() {
        assert_eq!(num_squares_dp(1), 1);
        assert_eq!(num_squares_dp(2), 2);
        assert_eq!(num_squares_dp(3), 3);
        assert_eq!(num_squares_dp(4), 1);
        assert_eq!(num_squares_dp(12), 3);
        assert_eq!(num_squares_dp(13), 2);
        assert_eq!(num_squares_dp(50), 2);
    }

    #[test]
    fn test_num_squares_req() {
        assert_eq!(num_squares_rec(1), 1);
        assert_eq!(num_squares_rec(2), 2);
        assert_eq!(num_squares_rec(3), 3);
        assert_eq!(num_squares_rec(4), 1);
    }

    #[test]
    fn test_num_squares_bfs() {
        assert_eq!(num_squares_bfs(1), 1);
        assert_eq!(num_squares_bfs(2), 2);
        assert_eq!(num_squares_bfs(3), 3);
        assert_eq!(num_squares_bfs(4), 1);
        assert_eq!(num_squares_bfs(12), 3);
        assert_eq!(num_squares_bfs(13), 2);
        assert_eq!(num_squares_bfs(50), 2);
    }
}

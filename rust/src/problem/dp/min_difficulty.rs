pub fn min_difficulty_td(job_difficulty: Vec<i32>, d: i32) -> i32 {
    if d > job_difficulty.len() as i32 {
        return -1;
    }

    let mut memo = vec![vec![-1; d as usize + 1]; job_difficulty.len()];

    fn dp(jobs: &[i32], memo: &mut Vec<Vec<i32>>, i: usize, d: usize) -> i32 {
        if d == 1 {
            return *jobs[i..].iter().max().unwrap_or(&0);
        }

        if memo[i][d] != -1 {
            return memo[i][d];
        }

        memo[i][d] = i32::MAX;

        for j in i..jobs.len() - d + 1 {
            memo[i][d] = memo[i][d].min(*jobs[i..=j].iter().max().unwrap() + dp(jobs, memo, j + 1, d - 1));
        }

        memo[i][d]
    }

    dp(&job_difficulty, &mut memo, 0, d as usize)
}

pub fn min_difficulty_bu(jobs: Vec<i32>, d: i32) -> i32 {
    if d > jobs.len() as i32 {
        return -1;
    }

    let mut dp = vec![vec![i32::MAX; jobs.len() + 1]; d as usize + 1];

    for i in 1..d as usize + 1 {
        for j in 0..jobs.len() - i + 1 {
            if i == 1 {
                dp[i][j] = *jobs[j..].iter().max().unwrap();
            } else {
                for k in j..jobs.len() - i + 1 {
                    dp[i][j] = dp[i][j].min(*jobs[j..=k].iter().max().unwrap() + dp[i - 1][k + 1]);
                }
            }
        }
    }

    dp[d as usize][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_difficulty_td() {
        assert_eq!(min_difficulty_td(vec![6, 5, 4, 3, 2, 1], 2), 7);
        assert_eq!(min_difficulty_td(vec![9, 9, 9], 4), -1);
        assert_eq!(min_difficulty_td(vec![3, 2, 1], 2), 4);
        assert_eq!(min_difficulty_td(vec![7, 1, 7, 1, 7, 1], 3), 15);
        assert_eq!(min_difficulty_td(vec![1, 1, 1], 3), 3);
    }

    #[test]
    fn test_min_difficulty_bu() {
        assert_eq!(min_difficulty_bu(vec![6, 5, 4, 3, 2, 1], 2), 7);
        assert_eq!(min_difficulty_bu(vec![9, 9, 9], 4), -1);
        assert_eq!(min_difficulty_bu(vec![3, 2, 1], 2), 4);
        assert_eq!(min_difficulty_bu(vec![7, 1, 7, 1, 7, 1], 3), 15);
        assert_eq!(min_difficulty_bu(vec![1, 1, 1], 3), 3);
    }
}
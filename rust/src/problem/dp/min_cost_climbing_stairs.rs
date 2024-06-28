pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut dp = vec![0; cost.len() + 1];
    dp[0] = cost[0];
    dp[1] = cost[1];

    for i in 2..dp.len() {
        if i == cost.len() {
            dp[i] = std::cmp::min(dp[i - 1], dp[i - 2]);
        } else {
            dp[i] = cost[i] + (std::cmp::min(dp[i - 1], dp[i - 2]));
        }
    }

    dp[dp.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_climbing_stairs() {
        assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]), 6);
    }
}
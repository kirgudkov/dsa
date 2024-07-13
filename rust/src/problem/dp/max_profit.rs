pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    let mut dp = vec![vec![vec![0; 2]; k as usize + 1]; prices.len() + 1];

    for i in (0..=prices.len() - 1).rev() {
        for tx in 1..=k as usize {
            for holding in 0..=1 {
                let do_nothing = dp[i + 1][tx][holding];

                let do_something = if holding == 1 {
                    // Sell
                    prices[i] + dp[i + 1][tx - 1][0]
                } else {
                    // Buy
                    -prices[i] + dp[i + 1][tx][1]
                };

                dp[i][tx][holding] = do_nothing.max(do_something);
            }
        }
    }

    dp[0][k as usize][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(2, vec![2, 4, 1]), 2);
        assert_eq!(max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    }
}
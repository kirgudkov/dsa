// https://leetcode.com/problems/coin-change
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![amount + 1; amount as usize + 1];
    dp[0] = 0;

    for i in 1..=amount as usize {
        for coin in &coins {
            if *coin <= i as i32 {
                dp[i] = dp[i].min(1 + dp[i - *coin as usize]);
            }
        }
    }

    if dp[amount as usize] > amount { -1 } else { dp[amount as usize] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(coin_change(vec![2], 3), -1);
        assert_eq!(coin_change(vec![1], 0), 0);
        assert_eq!(coin_change(vec![1], 1), 1);
        assert_eq!(coin_change(vec![1], 2), 2);
        assert_eq!(coin_change(vec![1, 2, 5], 100), 20);
        assert_eq!(coin_change(vec![186, 419, 83, 408], 6249), 20);
    }
}
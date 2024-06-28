pub fn tribonacci(n: i32) -> i32 {
    if n < 2 {
        return n;
    }

    let mut dp = vec![0; n as usize + 1];
    dp[0] = 0;
    dp[1] = 1;
    dp[2] = 1;

    for i in 3..n as usize + 1 {
        dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3];
    }

    dp[n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tribonacci() {
        assert_eq!(tribonacci(2), 1);
        assert_eq!(tribonacci(3), 2);
        assert_eq!(tribonacci(4), 4);
        assert_eq!(tribonacci(25), 1389537);
    }
}
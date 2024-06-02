pub fn fib(n: i32) -> i32 {
    fib_memo(n, &mut vec![-1; (n + 1) as usize])
}

fn fib_memo(n: i32, memo: &mut Vec<i32>) -> i32 {
    if memo[n as usize] != -1 {
        return memo[n as usize];
    }

    if n < 2 {
        return n;
    }

    let result = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);
    memo[n as usize] = result;

    result
}

#[cfg(test)]
mod tests {
    use crate::problem::fib::fib;

    #[test]
    fn test_fib() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(10), 55);
    }
}
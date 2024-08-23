pub fn fib(n: i32) -> i32 {
    _fib(n as usize, &mut vec![None; n as usize + 1])
}

fn _fib(n: usize, memo: &mut Vec<Option<i32>>) -> i32 {
    if let Some(cached) = memo[n] {
        return cached;
    }

    if n < 2 {
        return n as i32;
    }

    let result = _fib(n - 1, memo) + _fib(n - 2, memo);
    memo[n] = Some(result);

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
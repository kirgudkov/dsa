use std::collections::HashSet;

// https://leetcode.com/problems/happy-number
fn is_happy(mut n: i32) -> bool {
    let mut set = HashSet::new();

    let next = |mut n: i32| -> i32 {
        let mut next = 0;

        while n > 0 {
            next += (n % 10) * (n % 10);
            n /= 10;
        }

        next
    };

    // I haven't found any decent proof that this should work.
    while n != 1 && !set.contains(&n) {
        set.insert(n);
        n = next(n)
    }

    n == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert!(is_happy(19));
        assert!(!is_happy(2));
    }
}
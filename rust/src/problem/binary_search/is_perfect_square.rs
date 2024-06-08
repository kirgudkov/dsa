pub fn is_perfect_square(num: i32) -> bool {
    let mut l = 0;
    let mut r = ((num / 2) + 1).min(46340); // sqrt(i32::MAX) = 46340

    while l <= r {
        let m = l + (r - l) / 2;

        if m * m == num {
            return true;
        }

        if m * m > num {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(is_perfect_square(0));
        assert!(is_perfect_square(1));
        assert!(is_perfect_square(16));
        assert!(!is_perfect_square(14));
        assert!(!is_perfect_square(i32::MAX));
    }
}
pub fn my_sqrt(x: i32) -> i32 {
    let mut l = 1;
    let mut r = x.min(46340);

    while l <= r {
        let m = l + (r - l) / 2;

        if m * m == x {
            return m;
        }

        if x < m * m {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    l - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(my_sqrt(0), 0);
        assert_eq!(my_sqrt(1), 1);
        assert_eq!(my_sqrt(2), 1);
        assert_eq!(my_sqrt(3), 1);
        assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(5), 2);
        assert_eq!(my_sqrt(6), 2);
        assert_eq!(my_sqrt(7), 2);
        assert_eq!(my_sqrt(8), 2);
        assert_eq!(my_sqrt(9), 3);
        assert_eq!(my_sqrt(16), 4);
        assert_eq!(my_sqrt(25), 5);
        assert_eq!(my_sqrt(36), 6);
        assert_eq!(my_sqrt(49), 7);
        assert_eq!(my_sqrt(64), 8);
        assert_eq!(my_sqrt(81), 9);
        assert_eq!(my_sqrt(100), 10);
        assert_eq!(my_sqrt(2147395599), 46339);
    }
}
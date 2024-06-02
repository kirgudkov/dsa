pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 || x == 1.0 {
        return 1.0;
    }

    if n == 1 {
        return x;
    }

    if n == -1 {
        return 1.0 / x;
    }

    let mut result = my_pow(x, n / 2);
    result *= result;

    if n % 2 == 1 {
        result *= x;
    } else if n % 2 == -1 {
        result /= x;
    }

    fix(result)
}

fn fix(x: f64) -> f64 {
    (x * 100000.0).round() / 100000.0
}

#[cfg(test)]
mod tests {
    use crate::problem::my_pow::my_pow;

    #[test]
    fn test_my_pow() {
        assert_eq!(my_pow(2.00000, 10), 1024.00000);
        assert_eq!(my_pow(2.10000, 3), 9.26100);
        assert_eq!(my_pow(2.00000, -2), 0.25000);
        assert_eq!(my_pow(0.00001, 2147483647), 0.00000);
        assert_eq!(my_pow(-1.00001, 447125), -86.75668);
        assert_eq!(my_pow(1.00000, 2147483647), 1.00000);
        assert_eq!(my_pow(2.00000, -2147483648), 0.0);
    }
}
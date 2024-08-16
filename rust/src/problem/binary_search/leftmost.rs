fn leftmost<T: PartialOrd>(vec: &[T], target: &T) -> usize {
    let mut l = 0;
    let mut r = vec.len();

    while l < r {
        let m = l + (r - l) / 2;

        if &vec[m] >= target {
            r = m;
        } else {
            l = m + 1;
        }
    }

    l
}

fn rightmost<T: PartialOrd>(vec: &[T], target: &T) -> usize {
    let mut l = 0;
    let mut r = vec.len();

    while l < r {
        let m = l + (r - l) / 2;

        if &vec[m] > target {
            r = m;
        } else {
            l = m + 1;
        }
    }

    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leftmost() {
        assert_eq!(leftmost(&[1, 2, 2, 3], &2), 1);
        assert_eq!(leftmost(&[1, 2, 3], &2), 1);
        assert_eq!(leftmost(&[2, 3], &2), 0);
        assert_eq!(leftmost(&[1, 2], &2), 1);
        assert_eq!(leftmost(&[1, 3], &2), 1);
        assert_eq!(leftmost(&[], &2), 0);
    }

    #[test]
    fn test_rightmost() {
        assert_eq!(rightmost(&[1, 2, 2, 3], &2) - 1, 2);
        assert_eq!(rightmost(&[1, 2, 3], &2) - 1, 1);
        assert_eq!(rightmost(&[2, 3], &2) - 1, 0);
        assert_eq!(rightmost(&[1, 2], &2) - 1, 1);
        assert_eq!(rightmost(&[1, 3], &2), 1);
        assert_eq!(rightmost(&[], &2), 0);
    }
}

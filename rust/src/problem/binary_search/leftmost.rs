fn leftmost<T: Ord>(vec: &[T], target: &T) -> usize {
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

fn rightmost<T: Ord>(vec: &[T], target: &T) -> usize {
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

fn rightmost_alternative<T: Ord>(vec: &[T], target: &T) -> Option<usize> {
    let mut res = None;

    let mut l = 0i32;
    let mut r = vec.len() as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;

        if &vec[m as usize] <= target {
            if &vec[m as usize] == target {
                res = Some(m as usize);
            }
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    res
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

    #[test]
    fn test_rightmost_alternative() {
        assert_eq!(rightmost_alternative(&[1, 2, 2, 3], &2), Some(2));
        assert_eq!(rightmost_alternative(&[1, 2, 3], &2), Some(1));
        assert_eq!(rightmost_alternative(&[2, 3], &2), Some(0));
        assert_eq!(rightmost_alternative(&[1, 2], &2), Some(1));
        assert_eq!(rightmost_alternative(&[1, 3], &2), None);
        assert_eq!(rightmost_alternative(&[], &2), None);
    }
}

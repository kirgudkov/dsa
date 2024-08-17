use std::cmp::Ordering;

fn binary_search<T: Ord>(vec: &[T], target: T) -> Option<usize> {
    let mut l = 0;
    let mut r = vec.len();

    while l < r {
        let m = l + (r - l) / 2;

        match vec[m].cmp(&target) {
            Ordering::Less => {
                l = m + 1;
            }
            Ordering::Greater => {
                r = m;
            }
            Ordering::Equal => {
                return Some(m);
            }
        }
    }

    None
}

fn binary_search_alternative<T: Ord>(vec: &[T], target: T) -> Option<usize> {
    let mut l = 0;
    let mut r = vec.len() - 1;

    while l != r {
        let m = (l + r).div_ceil(2);

        if vec[m] > target {
            r = m - 1;
        } else {
            l = m;
        }
    }

    if vec[l] == target {
        Some(l)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(&[-1, 0, 3, 5, 9, 12], 9), Some(4));
        assert_eq!(binary_search(&[-1, 0, 3, 5, 9, 12], 2), None);
        assert_eq!(binary_search(&[-1, 0, 3, 5, 9, 12, 15], 12), Some(5));
        assert_eq!(binary_search(&[-1, 0, 3, 5, 9, 12, 15], 17), None);
        assert_eq!(binary_search(&[3, 5, 9, 12, 15], 1), None);
        assert_eq!(binary_search(&[5], 5), Some(0));
    }

    #[test]
    fn test_binary_search_alternative() {
        assert_eq!(binary_search_alternative(&[-1, 0, 3, 5, 9, 12], 9), Some(4));
        assert_eq!(binary_search_alternative(&[-1, 0, 3, 5, 9, 12], 2), None);
        assert_eq!(binary_search_alternative(&[-1, 0, 3, 5, 9, 12, 15], 12), Some(5));
        assert_eq!(binary_search_alternative(&[-1, 0, 3, 5, 9, 12, 15], 17), None);
        assert_eq!(binary_search_alternative(&[3, 5, 9, 12, 15], 1), None);
        assert_eq!(binary_search_alternative(&[5], 5), Some(0));
    }
}
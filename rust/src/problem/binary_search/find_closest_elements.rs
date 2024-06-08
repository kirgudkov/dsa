pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let mut l = 0;
    let mut r = arr.len() - k as usize;

    while l < r {
        let m = l + (r - l) / 2;

        if x - arr[m] > arr[m + k as usize] - x {
            l = m + 1;
        } else {
            r = m;
        }
    }

    arr[l..(l + k as usize)].into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3), vec![1, 2, 3, 4]);
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 0), vec![1, 2, 3, 4]);
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 1), vec![1, 2, 3, 4]);
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 2), vec![1, 2, 3, 4]);
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 4), vec![2, 3, 4, 5]);
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 5), vec![2, 3, 4, 5]);
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 7), vec![2, 3, 4, 5]);
        assert_eq!(find_closest_elements(vec![1, 1, 1, 10, 10, 10], 1, 9), vec![10]);
    }
}
pub fn find_kth_positive_bf(arr: Vec<i32>, mut k: i32) -> i32 {
    if k < arr[0] {
        return k;
    }

    k -= arr[0] - 1;

    for i in 0..arr.len() - 1 {
        let missing = arr[i + 1] - arr[i] - 1;

        if k <= missing {
            return arr[i] + k;
        }

        k -= missing;
    }

    arr.last().unwrap() + k
}

pub fn find_kth_positive_bs(arr: Vec<i32>, k: i32) -> i32 {
    let mut l = 0;
    let mut r = arr.len() - 1;

    while l <= r {
        let m = l + (r - l) / 2;

        if arr[m] - m as i32 - 1 < k {
            l = m + 1;
        } else { 
            r = m - 1;
        }
    }

    l as i32 + k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_positive_bf() {
        assert_eq!(find_kth_positive_bf(vec![2, 3, 4, 7, 11], 5), 9);
        assert_eq!(find_kth_positive_bf(vec![1, 2, 3, 4], 2), 6);
    }

    #[test]
    fn test_find_kth_positive_bs() {
        assert_eq!(find_kth_positive_bs(vec![2, 3, 4, 7, 11], 5), 9);
        assert_eq!(find_kth_positive_bs(vec![1, 2, 3, 4], 2), 6);
    }
}
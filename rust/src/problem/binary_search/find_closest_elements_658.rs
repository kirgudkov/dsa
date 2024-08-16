// https://leetcode.com/problems/find-k-closest-elements
pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let mut l = 0i32;
    let mut r = arr.len() as i32;

    // 1. Find closest element to x using binary search:
    while l < r {
        let m = l + (r - l) / 2;

        if arr[m as usize] >= x {
            r = m;
        } else {
            l = m + 1;
        }
    } // l is the leftmost closest to x;

    r = l;

    // 2. Expand window until it reaches size of k;
    while r - l <= k {
        if l < 0 {
            r += 1;
            continue;
        }

        if r as usize == arr.len() || (arr[l as usize] - x).abs() <= (arr[r as usize] - x).abs() {
            l -= 1;
        } else {
            r += 1;
        }
    }

    arr[(l + 1) as usize..r as usize].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 4), vec![2, 3, 4, 5]);
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3), vec![1, 2, 3, 4]);
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 0), vec![1, 2, 3, 4]);
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 1), vec![1, 2, 3, 4]);
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 2), vec![1, 2, 3, 4]);
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 5), vec![2, 3, 4, 5]);
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 7), vec![2, 3, 4, 5]);
        assert_eq!(find_closest_elements(vec![1, 1, 1, 10, 10, 10], 1, 9), vec![10]);
        assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8], 5, 4), vec![2, 3, 4, 5, 6]);
    }
}
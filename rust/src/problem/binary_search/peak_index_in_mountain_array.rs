// https://leetcode.com/problems/peak-index-in-a-mountain-array
pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = arr.len() - 1;

    while l < r {
        let m = l + (r - l) / 2;

        if arr[m + 1] >= arr[m] {
            l = m + 1;
        } else {
            r = m;
        }
    }

    l as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(peak_index_in_mountain_array(vec![0, 1, 0]), 1);
        assert_eq!(peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
        assert_eq!(peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
        assert_eq!(peak_index_in_mountain_array(vec![3, 4, 5, 1]), 2);
        assert_eq!(peak_index_in_mountain_array(vec![24, 69, 100, 99, 79, 78, 67, 36, 26, 19]), 2);
    }
}
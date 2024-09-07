// O(n+m) tc
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    median(merge(nums1, nums2))
}

fn merge(vec_1: Vec<i32>, vec_2: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    let mut i = 0;
    let mut j = 0;

    while i < vec_1.len() && j < vec_2.len() {
        if vec_1[i] <= vec_2[j] {
            result.push(vec_1[i]);
            i += 1;
        } else {
            result.push(vec_2[j]);
            j += 1;
        }
    }

    while i < vec_1.len() {
        result.push(vec_1[i]);
        i += 1;
    }

    while j < vec_2.len() {
        result.push(vec_2[j]);
        j += 1;
    }

    result
}

fn median(vec: Vec<i32>) -> f64 {
    let mid = vec.len() / 2;

    if vec.len() % 2 == 0 {
        (vec[mid] + vec[mid - 1]) as f64 / 2.0
    } else {
        vec[mid] as f64
    }
}

#[cfg(test)]
mod tests {
    use crate::problem::misc::median_of_two_sorted_arrays::{find_median_sorted_arrays};

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(find_median_sorted_arrays(vec![1, 2, 3, 6], vec![4, 5]), 3.5);
    }
}

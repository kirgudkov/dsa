use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: Vec<i32>, mut k: i32) -> i32 {
    let mut heap = BinaryHeap::from(nums);
    let mut result = 0;

    while let Some(value) = heap.pop() {
        if k == 1 {
            result = value;
            break;
        }

        k -= 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::problem::kth_largest_element::find_kth_largest;

    #[test]
    fn test_kth_largest() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }
}

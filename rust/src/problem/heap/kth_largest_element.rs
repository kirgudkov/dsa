use std::collections::BinaryHeap;

fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap = BinaryHeap::from(nums);

    (0..k - 1).for_each(|_| {
        heap.pop();
    });

    heap.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::problem::heap::kth_largest_element::find_kth_largest;

    #[test]
    fn test_kth_largest() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }
}

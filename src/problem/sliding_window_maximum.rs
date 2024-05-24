use std::collections::BinaryHeap;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let mut left = 0;
    let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();

    // tc: O(n)
    for (idx, &num) in nums.iter().enumerate() {

        // tc: O(log k)
        heap.push((num, idx));

        if idx < k as usize - 1 {
            continue;
        }

        // tc: O(n * log k)
        // remove every root that is outside the window
        while heap.peek().unwrap().1 < left {
            heap.pop();
        }

        result.push(heap.peek().unwrap().0);

        left += 1;
    }
    // total tc: O(n * log k)
    result
}

#[cfg(test)]
mod tests {
    use crate::problem::sliding_window_maximum::max_sliding_window;

    #[test]
    fn test() {
        assert_eq!(max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
    }
}

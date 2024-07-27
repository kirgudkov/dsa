use std::collections::BinaryHeap;

// https://leetcode.com/problems/sliding-window-maximum
// TC O(nlogn) SC O(n)
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let mut left = 0;
    let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();

    for (i, &num) in nums.iter().enumerate() {
        heap.push((num, i));

        if i < k as usize - 1 {
            continue;
        }

        // Heap size not necessary should be k, but if heap contains max whose index is less then current l,
        // that means it doesn't belong to the current window and should be removed;
        while heap.peek().unwrap().1 < left {
            heap.pop();
        }

        result.push(heap.peek().unwrap().0);
        left += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::problem::heap::sliding_window_maximum_239::max_sliding_window;

    #[test]
    fn test() {
        assert_eq!(max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
    }
}

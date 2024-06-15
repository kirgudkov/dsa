use std::cmp::Reverse;
use std::collections::BinaryHeap;

// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix
// Time complexity: O(x + k log x), where x is min(n, k)
// Not really obvious intuition. Didn't come up with this solution myself, stole it from editorial.
// The idea is that each row is a sorted list, so we can perform kind of a "virtual" merge sort on all of them.
// We use min heap to keep track of n pointers, one for each row.
// While k > 0, we pop the smallest element from the heap, and push the next element from the same row.
// Once k == 0, we have the kth smallest element at the top of the heap.
pub fn kth_smallest(matrix: Vec<Vec<i32>>, mut k: i32) -> i32 {
    let mut heap = BinaryHeap::new();

    for (i, row) in matrix.iter().take(matrix.len().min(k as usize)).enumerate() {
        heap.push(Reverse((row[0], i, 0)));
    }

    k -= 1; // 0-indexed
    let mut kth = heap.peek().unwrap().0.0;

    while k > 0 {
        let Reverse((_, row, col)) = heap.pop().unwrap();

        if col < matrix[0].len() - 1 {
            heap.push(Reverse((matrix[row][col + 1], row, col + 1)));
        }

        kth = heap.peek().unwrap().0.0;

        k -= 1;
    }

    kth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_smallest() {
        let matrix = vec![
            vec![1, 5, 9],
            vec![10, 11, 13],
            vec![12, 13, 15],
        ];
        let k = 8;
        assert_eq!(kth_smallest(matrix, k), 13);

        let matrix = vec![
            vec![1, 3, 5],
            vec![6, 7, 12],
            vec![11, 14, 14],
        ];
        let k = 3;
        assert_eq!(kth_smallest(matrix, k), 5);

        let matrix = vec![
            vec![-5],
        ];

        let k = 1;
        assert_eq!(kth_smallest(matrix, k), -5);
    }
}
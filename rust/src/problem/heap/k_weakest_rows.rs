use std::cmp::Reverse;
use std::collections::BinaryHeap;

// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix
// TC is O(m * n + m * log m), where m is the number of rows in matrix
pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut heap = BinaryHeap::new();

    for (i, row) in mat.iter().enumerate() {
        let soldiers_count = row.iter().take_while(|&&x| x == 1).count();
        heap.push(Reverse((soldiers_count, i)));
    }

    let mut res = Vec::with_capacity(k as usize);

    for _ in 0..k {
        if let Some(Reverse((_, index))) = heap.pop() {
            res.push(index as i32);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_weakest_rows() {
        let mat = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
        ];
        let k = 3;
        let res = vec![2, 0, 3];
        assert_eq!(k_weakest_rows(mat, k), res);
    }
}
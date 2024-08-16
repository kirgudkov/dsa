use std::cmp::Reverse;
use std::collections::BinaryHeap;

// https://leetcode.com/problems/k-closest-points-to-origin
// TC is O(n log n)
// SC is O(n)
pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut heap = BinaryHeap::new();

    fn distance(point: &[i32]) -> i32 {
        // There is no need to calculate the square root since we are only interested in the relative distance
        // + we don't need to subtract the origin point since it is (0, 0)
        point[0] * point[0] + point[1] * point[1]
    }

    for point in points {
        heap.push(Reverse((distance(&point), point)));
    }

    let mut result = Vec::new();

    for _ in 0..k {
        result.push(heap.pop().unwrap().0.1);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_closest() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let k = 1;
        let result = vec![vec![-2, 2]];
        assert_eq!(k_closest(points, k), result);

        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let k = 2;
        let result = vec![vec![3, 3], vec![-2, 4]];
        assert_eq!(k_closest(points, k), result);
    }
}
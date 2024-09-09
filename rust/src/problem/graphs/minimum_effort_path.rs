use crate::utils::Neighbors;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let mut visited = vec![vec![false; heights[0].len()]; heights.len()];
    let mut heap = BinaryHeap::from([Reverse((0, 0, 0))]);

    while let Some(Reverse((effort, i, j))) = heap.pop() {
        if i == heights.len() - 1 && j == heights[0].len() - 1 {
            return effort;
        }

        if visited[i][j] {
            continue;
        }

        visited[i][j] = true;

        for (ni, nj) in heights.neighbors(i, j) {
            if !visited[ni][nj] {
                heap.push(Reverse((effort.max((heights[i][j] - heights[ni][nj]).abs()), ni, nj)));
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_effort_path() {
        let heights = vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]];
        assert_eq!(minimum_effort_path(heights), 2);

        let heights = vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]];
        assert_eq!(minimum_effort_path(heights), 1);

        let heights = vec![vec![1, 2, 1, 1, 1], vec![1, 2, 1, 2, 1], vec![1, 2, 1, 2, 1], vec![1, 2, 1, 2, 1], vec![1, 1, 1, 2, 1]];
        assert_eq!(minimum_effort_path(heights), 0);
    }
}
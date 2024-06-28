use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let mut visited = vec![vec![false; heights[0].len()]; heights.len()];
    let mut heap = BinaryHeap::from([Reverse((0, 0, 0))]);

    while let Some(Reverse((effort, i, j))) = heap.pop() {
        if visited[i][j] {
            continue;
        }

        if i == heights.len() - 1 && j == heights[0].len() - 1 {
            return effort;
        }

        visited[i][j] = true;

        for (di, dj) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let new_i = (i as i32 + di) as usize;
            let new_j = (j as i32 + dj) as usize;

            if new_i < heights.len() && new_j < heights[0].len() && !visited[new_i][new_j] {
                let new_effort = (heights[i][j] - heights[new_i][new_j]).abs();
                heap.push(Reverse((effort.max(new_effort), new_i, new_j)));
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
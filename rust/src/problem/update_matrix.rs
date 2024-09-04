use crate::utils::Neighbors;
use std::collections::VecDeque;

// https://leetcode.com/problems/01-matrix
pub fn update_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();

    for (i, row) in matrix.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if *cell == 0 {
                queue.push_back((i, j, 0));
            } else {
                *cell = i32::MAX;
            }
        }
    }

    while let Some((i, j, distance)) = queue.pop_front() {
        if matrix[i][j] != 0 {
            matrix[i][j] = matrix[i][j].min(distance as i32);
        }

        for (i, j) in matrix.neighbors(i, j) {
            if matrix[i][j] == i32::MAX {
                queue.push_back((i, j, distance + 1));
            }
        }
    }

    matrix
}


#[cfg(test)]
mod tests {
    use crate::problem::update_matrix::update_matrix;

    #[test]
    fn test_update_matrix() {
        assert_eq!(
            update_matrix(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]);

        assert_eq!(
            update_matrix(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![1, 1, 1]
            ]),
            vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![1, 2, 1]
            ]);

        assert_eq!(
            update_matrix(vec![
                vec![0, 1, 1, 0, 0],
                vec![0, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 1],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 1, 0],
            ]),
            vec![
                vec![0, 1, 1, 0, 0],
                vec![0, 1, 1, 0, 0],
                vec![0, 1, 0, 0, 1],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 1, 0],
            ])
    }
}

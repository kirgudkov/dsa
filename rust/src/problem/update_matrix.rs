use std::collections::VecDeque;

// https://leetcode.com/problems/01-matrix
pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (h, w) = (mat.len(), mat[0].len());

    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();

    for (i, row) in mat.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if *cell == 0 {
                queue.push_back((i, j, 0));
            } else {
                *cell = i32::MAX;
            }
        }
    }

    while let Some((i, j, distance)) = queue.pop_front() {
        if mat[i][j] != 0 {
            mat[i][j] = mat[i][j].min(distance as i32);
        }

        for &(delta_i, delta_j) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_i = (i as isize + delta_i) as usize;
            let new_j = (j as isize + delta_j) as usize;

            if new_i < h && new_j < w && mat[new_i][new_j] == i32::MAX {
                queue.push_back((new_i, new_j, distance + 1));
            }
        }
    }

    mat
}

#[cfg(test)]
mod tests {
    use crate::problem::update_matrix::update_matrix;

    #[test]
    fn test_update_matrix() {
        assert_eq!(update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]), vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]);
        assert_eq!(update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]), vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]);
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

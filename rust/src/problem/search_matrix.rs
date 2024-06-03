pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() {
        return false;
    }

    let top_left = Cell { i: 0, j: 0 };
    let bottom_right = Cell { i: matrix.len() - 1, j: matrix[0].len() - 1 };

    search_sub_matrix(&matrix, target, top_left, bottom_right)
}

fn search_sub_matrix(matrix: &Vec<Vec<i32>>, target: i32, top_left: Cell, bottom_right: Cell) -> bool {
    // Base case: matrix consists of a single element:
    if top_left.i == bottom_right.i && top_left.j == bottom_right.j {
        return matrix[top_left.i][top_left.j] == target;
    }

    // Top-left element is matrix's minimum and bottom-right element is matrix's maximum;
    // Thus, if target is out of this range, it's not in the matrix:
    if matrix[top_left.i][top_left.j] > target || matrix[bottom_right.i][bottom_right.j] < target {
        return false;
    }

    // Recursively divide the matrix into 4 sub-matrices and exclude two of them:
    // top-left and bottom-right.
    // Essentially it is two-dimensional binary search.

    // Pick the vertical split point:
    let mid_col = (top_left.j + bottom_right.j) / 2;

    // Traverse mid-column and try to find a horizontal split point, which is
    // the cell with the value greater than the target.
    for i in top_left.i..=bottom_right.i {
        // if we're lucky enough to find the target in the mid-column, we're done:
        if matrix[i][mid_col] == target {
            return true;
        }

        // if we found the cell with the value greater than the target, we can split the matrix vertically;
        // Run search recursively on the bottom-left and top-right sub-matrices:
        if matrix[i][mid_col] > target {
            let top_right = {
                let top_left = Cell { i: top_left.i, j: mid_col };
                let bottom_right = Cell { i: i.wrapping_sub(1), j: bottom_right.j };

                search_sub_matrix(matrix, target, top_left, bottom_right)
            };

            let bottom_left = {
                let top_left = Cell { i, j: top_left.j };
                let bottom_right = Cell { i: bottom_right.i, j: mid_col.wrapping_sub(1) };

                search_sub_matrix(matrix, target, top_left, bottom_right)
            };

            return top_right || bottom_left;
        }
    }

    // We couldn't split the matrix horizontally; Search in the right full-height sub-matrix:
    search_sub_matrix(matrix, target, Cell { i: top_left.i, j: mid_col + 1 }, bottom_right)
}

struct Cell {
    i: usize,
    j: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix() {
        assert!(search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 60));
        assert!(search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 3));
        assert!(!search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 13));

        assert!(search_matrix(vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]], 5));
        assert!(search_matrix(vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]], 13));
        assert!(!search_matrix(vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]], 20));
        assert!(!search_matrix(vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]], 0));

        assert!(search_matrix(vec![vec![-1, 3]], -1));
        assert!(search_matrix(vec![vec![-1, 3]], 3));

        assert!(search_matrix(vec![vec![1, 4], vec![2, 5]], 5));

        assert!(search_matrix(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25]], 23));
        assert!(search_matrix(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25]], 24));
        assert!(search_matrix(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25]], 25));
    }
}

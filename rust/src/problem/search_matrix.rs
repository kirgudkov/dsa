pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() {
        return false;
    }

    let tl = Cell { i: 0, j: 0 };
    let br = Cell { i: matrix.len() - 1, j: matrix[0].len() - 1 };

    search_sub_matrix(&matrix, target, tl, br)
}

fn search_sub_matrix(matrix: &Vec<Vec<i32>>, target: i32, tl: Cell, br: Cell) -> bool {
    // Base case: matrix consists of a single element:
    if tl.i == br.i && tl.j == br.j {
        return matrix[tl.i][tl.j] == target;
    }

    // top-left element is matrix's minimum and bottom-right element is matrix's maximum;
    // Thus, if target is out of this range, it's not in the matrix:
    if matrix[tl.i][tl.j] > target || matrix[br.i][br.j] < target {
        return false;
    }

    // Recursively divide the matrix into 4 sub-matrices and exclude two of them:
    // top-left and bottom-right.
    // Essentially it is two-dimensional binary search.

    // Split vertically:
    let mid_h = (tl.j + br.j) / 2;
    let mut i = tl.i;

    // Try to find the row where target is located or cell is greater than target:
    while i <= br.i {
        if matrix[i][mid_h] == target {
            return true;
        }

        if matrix[i][mid_h] > target {
            if i == tl.i {
                return search_sub_matrix(matrix, target, tl, Cell { i: br.i, j: mid_h - 1 });
            }

            return search_sub_matrix(matrix, target, Cell { i: tl.i, j: mid_h }, Cell { i: i - 1, j: br.j })
                || search_sub_matrix(matrix, target, Cell { i, j: tl.j }, Cell { i: br.i, j: mid_h - 1 });
        }

        i += 1;
    }

    // We couldn't split the matrix vertically; Search in the right full-height sub-matrix:
    search_sub_matrix(matrix, target, Cell { i: tl.i, j: mid_h + 1 }, br)
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

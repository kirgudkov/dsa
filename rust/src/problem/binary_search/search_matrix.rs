pub fn search_matrix(matrix: &[Vec<i32>], target: i32) -> bool {
    if target < matrix[0][0] || target > matrix[matrix.len() - 1][matrix[0].len() - 1] {
        return false;
    }

    // 1. do binary search to find target row
    let mut i = 0i32;
    let mut r = matrix.len() as i32 - 1;

    while i <= r {
        let m = i + (r - i) / 2;

        if target >= matrix[m as usize][0] && target <= matrix[m as usize][matrix[0].len() - 1] {
            i = m;
            break;
        }

        if target > matrix[m as usize][matrix[0].len() - 1] {
            i = m + 1;
        }

        if target < matrix[m as usize][0] {
            r = m - 1;
        }
    }

    // 2. do bs in this row to find target
    let mut l = 0i32;
    r = matrix[i as usize].len() as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;

        if matrix[i as usize][m as usize] == target {
            return true;
        }

        if target < matrix[i as usize][m as usize] {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix() {
        assert!(search_matrix(&[vec![1], vec![3], vec![5]], 1));

        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(search_matrix(&matrix, 11));
        assert!(search_matrix(&matrix, 3));
        assert!(!search_matrix(&matrix, 13));
        assert!(search_matrix(&matrix, 60));
        assert!(!search_matrix(&matrix, 0));
    }
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];

    let mut i = 0;
    let mut j = 0;

    let mut direction = Direction::Right;

    let mut r_boundary = matrix[0].len() - 1;
    let mut d_boundary = matrix.len() - 1;
    let mut l_boundary = 0;
    let mut u_boundary = 0;

    while res.len() != matrix.len() * matrix[0].len() {
        res.push(matrix[i][j]);

        match direction {
            Direction::Right => {
                if j == r_boundary {
                    u_boundary += 1;
                    direction = Direction::Down;
                    i += 1;
                } else {
                    j += 1;
                }
            }
            Direction::Down => {
                if i == d_boundary {
                    r_boundary -= 1;
                    direction = Direction::Left;
                    j -= 1;
                } else {
                    i += 1;
                }
            }
            Direction::Left => {
                if j == l_boundary {
                    d_boundary -= 1;
                    direction = Direction::Up;
                    i -= 1;
                } else {
                    j -= 1;
                }
            }
            Direction::Up => {
                if i == u_boundary {
                    l_boundary += 1;
                    direction = Direction::Right;
                    j += 1;
                } else {
                    i -= 1;
                }
            }
        }
    }

    res
}

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_order() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);

        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let result = spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);

        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12]];
        let result = spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 3, 6, 9, 12, 11, 10, 7, 4, 5, 8]);
    }
}

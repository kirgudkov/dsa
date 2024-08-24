use crate::utils::neighbors;
use std::collections::VecDeque;

// https://leetcode.com/problems/flood-fill
pub fn flood_fill(mut image: Vec<Vec<i32>>, si: i32, sj: i32, color: i32) -> Vec<Vec<i32>> {
    if image[si as usize][sj as usize] == color {
        return image;
    }

    let original_color = image[si as usize][sj as usize];
    let mut queue = VecDeque::from([(si as usize, sj as usize)]);

    while let Some((i, j)) = queue.pop_front() {
        image[i][j] = color;

        neighbors(i, j, &image).iter().for_each(|&(i, j)| {
            if image[i][j] == original_color {
                queue.push_back((i, j))
            }
        });
    }

    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_fill() {
        assert_eq!(flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2), vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]);
        assert_eq!(flood_fill(vec![vec![0, 0, 0], vec![0, 1, 1]], 1, 1, 1), vec![vec![0, 0, 0], vec![0, 1, 1]]);
        assert_eq!(flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 2), vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]]);
        assert_eq!(flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0), vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]);
    }
}

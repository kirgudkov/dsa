// https://leetcode.com/problems/flood-fill
pub fn flood_fill(mut image: Vec<Vec<i32>>, si: i32, sj: i32, color: i32) -> Vec<Vec<i32>> {
    if image[si as usize][sj as usize] == color {
        return image;
    }

    let mut queue = std::collections::VecDeque::from([(si as usize, sj as usize)]);
    let target_color = image[si as usize][sj as usize];
    let (h, w) = (image.len(), image[0].len());

    while !queue.is_empty() {
        let (i, j) = queue.pop_front().unwrap();
        image[i][j] = color;

        if let Some(i) = i.checked_sub(1) {
            if image[i][j] == target_color {
                queue.push_back((i, j))
            }
        }

        if let Some(j) = j.checked_sub(1) {
            if image[i][j] == target_color {
                queue.push_back((i, j))
            }
        }

        if (i + 1) < h && image[i + 1][j] == target_color {
            queue.push_back((i + 1, j))
        }

        if (j + 1) < w && image[i][j + 1] == target_color {
            queue.push_back((i, j + 1))
        }
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

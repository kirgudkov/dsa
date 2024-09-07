use crate::utils::Neighbors;

// https://leetcode.com/problems/walls-and-gates
// TC/SC is O(mn);
// BFS approach: Starting from each 0-cell (gate) propagate outwards and increment distance in each neighbor cell
pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
    let mut q = std::collections::VecDeque::new();

    for (i, col) in rooms.iter().enumerate() {
        for (j, row) in col.iter().enumerate() {
            if *row == 0 {
                q.push_back((i, j, 0));
            }
        }
    }

    while let Some((i, j, distance)) = q.pop_front() {
        rooms[i][j] = rooms[i][j].min(distance);

        for (i, j) in rooms.neighbors(i, j) {
            if rooms[i][j] == i32::MAX {
                q.push_back((i, j, distance + 1));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walls_and_gates() {
        let mut rooms = vec![
            vec![i32::MAX, -1, 0, i32::MAX],
            vec![i32::MAX, i32::MAX, i32::MAX, -1],
            vec![i32::MAX, -1, i32::MAX, -1],
            vec![0, -1, i32::MAX, i32::MAX],
        ];
        walls_and_gates(&mut rooms);
        assert_eq!(rooms, vec![
            vec![3, -1, 0, 1],
            vec![2, 2, 1, -1],
            vec![1, -1, 2, -1],
            vec![0, -1, 3, 4],
        ]);
    }
}

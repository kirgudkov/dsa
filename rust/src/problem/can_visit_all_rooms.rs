use std::collections::VecDeque;

pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    let mut queue: VecDeque<(&Vec<i32>, usize)> = VecDeque::from([(&rooms[0], 0)]);
    let mut visited: Vec<usize> = vec![];

    while let Some((keys, room)) = queue.pop_front() {
        if visited.contains(&room) {
            continue;
        }

        visited.push(room);

        for key in keys.iter() {
            queue.push_back((&rooms[*key as usize], *key as usize));
        }
    }

    visited.len() == rooms.len()
}

#[cfg(test)]
mod tests {
    use crate::problem::can_visit_all_rooms::can_visit_all_rooms;

    #[test]
    fn test_can_visit_all_rooms() {
        assert!(can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]));
        assert!(!can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]]));
    }
}

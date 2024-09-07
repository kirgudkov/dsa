use crate::ds::disjoint_set::DisjointSet;

// https://leetcode.com/problems/number-of-provinces
pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut ds = DisjointSet::with_capacity(is_connected.len());
    let mut count = is_connected.len() as i32;

    for i in 0..is_connected.len() - 1 {
        for j in i + 1..is_connected.len() {
            if is_connected[i][j] == 1 && ds.union(i, j) {
                count -= 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_circle_num() {
        assert_eq!(find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]), 3);
        assert_eq!(find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]), 2);
        assert_eq!(find_circle_num(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]), 1);
    }
}
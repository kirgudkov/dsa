// https://leetcode.com/problems/number-of-provinces
pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut ds = DisjointSet::with_capacity(is_connected.len());

    for i in 0..is_connected.len() {
        for j in (i + 1)..is_connected.len() {
            if is_connected[i][j] == 1 {
                ds.union(i, j);
            }
        }
    }

    ds.count
}

pub struct DisjointSet {
    root: Vec<usize>,
    rank: Vec<usize>,
    count: i32,
}

impl DisjointSet {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            root: (0..capacity).collect(),
            rank: vec![0; capacity],
            count: capacity as i32,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.root[x] == x {
            return x;
        }

        self.root[x] = self.find(self.root[x]);
        self.root[x]
    }

    pub(crate) fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.root[root_x] = root_y;
            }
            std::cmp::Ordering::Greater => {
                self.root[root_y] = root_x;
            }
            std::cmp::Ordering::Equal => {
                self.root[root_x] = root_y;
                self.rank[root_y] += 1;
            }
        }

        self.count -= 1;
    }
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
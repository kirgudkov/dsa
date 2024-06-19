pub struct DisjointSet {
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    pub fn with_capacity(capacity: usize) -> Self {
        DisjointSet {
            root: (0..capacity).collect(),
            rank: vec![0; capacity],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.root[x] == x {
            return x;
        }

        self.root[x] = self.find(self.root[x]);
        self.root[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
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

        true
    }

    pub fn all_connected(&mut self) -> bool {
        let root = self.find(0);

        for i in 1..self.root.len() {
            if self.find(i) != root {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let ds = DisjointSet::with_capacity(5);
        assert_eq!(ds.root, vec![0, 1, 2, 3, 4]);
        assert_eq!(ds.rank, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_find() {
        let mut ds = DisjointSet::with_capacity(5);
        assert_eq!(ds.find(0), 0);
        assert_eq!(ds.find(1), 1);
        assert_eq!(ds.find(2), 2);
    }

    #[test]
    fn test_union() {
        let mut ds = DisjointSet::with_capacity(5);
        ds.union(0, 1);
        assert_eq!(ds.find(0), ds.find(1));

        ds.union(1, 2);
        assert_eq!(ds.find(0), ds.find(2));

        // Test rank updates
        let x = ds.find(0);
        assert_eq!(ds.rank[x], 1);
    }

    #[test]
    fn test_union_by_rank() {
        let mut ds = DisjointSet::with_capacity(5);
        ds.union(0, 1);
        ds.union(2, 3);
        ds.union(3, 4);

        let x = ds.find(3);
        assert_eq!(ds.rank[x], 1);
        let x = ds.find(4);
        assert_eq!(ds.rank[x], 1);

        ds.union(0, 2);

        let x = ds.find(0);
        assert_eq!(ds.rank[x], 2); // Since we connected two trees of rank 1
        assert_eq!(ds.find(1), ds.find(3));
        assert_eq!(ds.find(0), ds.find(4));
    }
}

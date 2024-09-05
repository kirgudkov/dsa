use std::cmp::Ordering;

pub struct DisjointSet {
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            root: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.root[x] != x {
            self.root[x] = self.find(self.root[x]);
        }

        self.root[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
            return false;
        }

        match self.rank[x].cmp(&self.rank[y]) {
            Ordering::Less => {
                self.root[x] = y
            }
            Ordering::Greater => {
                self.root[y] = x
            }
            Ordering::Equal => {
                self.root[x] = y;
                self.rank[y] += 1;
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
        assert_eq!(ds.rank[x], 2);
        assert_eq!(ds.find(1), ds.find(3));
        assert_eq!(ds.find(0), ds.find(4));
    }
}

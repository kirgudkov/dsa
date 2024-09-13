use std::cmp::Ordering;

pub struct DisjointSet {
    roots: Vec<usize>,
    ranks: Vec<usize>,
}

impl DisjointSet {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            roots: (0..n).collect(),
            ranks: vec![0; n],
        }
    }

    pub fn find(&mut self, v: usize) -> usize {
        if self.roots[v] != v {
            self.roots[v] = self.find(self.roots[v]);
        }

        self.roots[v]
    }

    pub fn union(&mut self, v: usize, u: usize) -> bool {
        let v_root = self.find(v);
        let u_root = self.find(u);

        if v_root == u_root {
            return false;
        }

        match self.ranks[v_root].cmp(&self.ranks[u_root]) {
            Ordering::Less => {
                self.roots[v_root] = u_root
            }
            Ordering::Greater => {
                self.roots[u_root] = v_root
            }
            Ordering::Equal => {
                self.roots[v_root] = u_root;
                self.ranks[u_root] += 1;
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
        assert_eq!(ds.roots, vec![0, 1, 2, 3, 4]);
        assert_eq!(ds.ranks, vec![0, 0, 0, 0, 0]);
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
        assert_eq!(ds.ranks[x], 1);
    }

    #[test]
    fn test_union_by_rank() {
        let mut ds = DisjointSet::with_capacity(5);
        ds.union(0, 1);
        ds.union(2, 3);
        ds.union(3, 4);

        let x = ds.find(3);
        assert_eq!(ds.ranks[x], 1);
        let x = ds.find(4);
        assert_eq!(ds.ranks[x], 1);

        ds.union(0, 2);

        let x = ds.find(0);
        assert_eq!(ds.ranks[x], 2);
        assert_eq!(ds.find(1), ds.find(3));
        assert_eq!(ds.find(0), ds.find(4));
    }
}

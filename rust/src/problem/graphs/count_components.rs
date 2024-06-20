use std::cmp::Ordering;

pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut ds = DisjointSet::with_capacity(n as usize);
    let mut count = n;

    for edge in edges {
        if ds.union(edge[0] as usize, edge[1] as usize) {
            count -= 1;
        }
    }

    count
}

struct DisjointSet {
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            root: (0..capacity).collect(),
            rank: vec![0; capacity],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.root[x] == x {
            return x;
        }

        self.root[x] = self.find(self.root[x]);
        self.root[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return false;
        }

        match self.rank[root_a].cmp(&self.rank[root_b]) {
            Ordering::Less => {
                self.root[root_a] = root_b;
            }
            Ordering::Greater => {
                self.root[root_b] = root_a;
            }
            Ordering::Equal => {
                self.root[root_a] = root_b;
                self.rank[root_b] += 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(count_components(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]), 1);
        assert_eq!(count_components(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]]), 1);
        assert_eq!(count_components(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]]), 2);
        assert_eq!(count_components(4, vec![vec![0, 1], vec![2, 3], vec![1, 2]]), 1);
    }
}

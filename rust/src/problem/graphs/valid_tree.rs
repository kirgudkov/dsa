use crate::ds::disjoint_set::DisjointSet;

pub fn valid_tree(mut n: i32, edges: Vec<Vec<i32>>) -> bool {
    if edges.len() as i32 != n - 1 {
        return false;
    }

    let mut ds = DisjointSet::with_capacity(n as usize);

    edges.iter().for_each(|edge| {
        if ds.union(edge[0] as usize, edge[1] as usize) {
            n -= 1;
        }
    });

    n == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(valid_tree(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]));
        assert!(!valid_tree(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]]));
        assert!(valid_tree(4, vec![vec![0, 1], vec![2, 3], vec![1, 2]]));
        assert!(!valid_tree(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]]));
    }
}
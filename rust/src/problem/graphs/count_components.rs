use crate::ds::disjoint_set::DisjointSet;

pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut ds = DisjointSet::with_capacity(n as usize);

    edges.iter().fold(n, |n, edge| {
        if ds.union(edge[0] as usize, edge[1] as usize) {
            n - 1
        } else { n }
    })
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

use crate::ds::disjoint_set::DisjointSet;

// https://leetcode.com/problems/min-cost-to-connect-all-points
// Min Cost to Connect All Points.

// Kruskal's MST approach.
// TC: O(n^2 * log(n)), SC: O(n^2), where n is the number of points.
pub fn min_cost_connect_points_krus(points: Vec<Vec<i32>>) -> i32 {
    let mut edges = vec![];

    for (a, p1) in points.iter().enumerate() {
        for (b, p2) in points.iter().enumerate().skip(a) {
            edges.push(((p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs(), a, b));
        }
    }

    edges.sort_unstable();

    let mut ds = DisjointSet::with_capacity(points.len());
    let mut min_cost = 0;
    let mut edges_count = 0;

    for (cost, a, b) in edges {
        if ds.union(a, b) {
            min_cost += cost;
            edges_count += 1;
        }

        if edges_count == points.len() - 1 {
            break;
        }
    }

    min_cost
}

// Prim's MST approach.
// TC: O(n^2 * log(n)), SC: O(n^2), where n is the number of points.
pub fn min_cost_connect_points_prim(points: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut min_heap = BinaryHeap::from([Reverse((0, 0))]);
    let mut visited = vec![false; points.len()];
    let mut points_count = 0;
    let mut min_cost = 0;

    while let Some(Reverse((cost, a))) = min_heap.pop() {
        if visited[a] {
            continue;
        }

        visited[a] = true;
        points_count += 1;
        min_cost += cost;

        if points_count == points.len() {
            break;
        }

        points.iter().enumerate().for_each(|(b, point)| {
            if !visited[b] {
                min_heap.push(Reverse(((point[0] - points[a][0]).abs() + (point[1] - points[a][1]).abs(), b)));
            }
        });
    }

    min_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_connect_points_krus() {
        assert_eq!(min_cost_connect_points_krus(vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]]), 20);
        assert_eq!(min_cost_connect_points_krus(vec![vec![3, 12], vec![-2, 5], vec![-4, 1]]), 18);
        assert_eq!(min_cost_connect_points_krus(vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![-1, 1]]), 4);
        assert_eq!(min_cost_connect_points_krus(vec![vec![-1000000, -1000000], vec![1000000, 1000000]]), 4000000);
        assert_eq!(min_cost_connect_points_krus(vec![vec![0, 0]]), 0);
    }

    #[test]
    fn test_min_cost_connect_points_prim() {
        assert_eq!(min_cost_connect_points_prim(vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]]), 20);
        assert_eq!(min_cost_connect_points_prim(vec![vec![3, 12], vec![-2, 5], vec![-4, 1]]), 18);
        assert_eq!(min_cost_connect_points_prim(vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![-1, 1]]), 4);
        assert_eq!(min_cost_connect_points_prim(vec![vec![-1000000, -1000000], vec![1000000, 1000000]]), 4000000);
        assert_eq!(min_cost_connect_points_prim(vec![vec![0, 0]]), 0);
    }
}

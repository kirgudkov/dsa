use crate::ds::disjoint_set::DisjointSet;

// https://leetcode.com/problems/min-cost-to-connect-all-points
// Min Cost to Connect All Points.

// Kruskal's algorithm with disjoint set aka union-find:
// 1. Create a list of edges with distance between each pair of points.
// 2. Sort the edges by distance.
// 3. Iterate over the edges and add the distance to the cost if the two points are not creating a cycle:
//    - To detect a cycle, use disjoint set to union two points
//    - If the two points are already in the same set (union() returned false), skip the edge
// 4. Return the cost.
//
// TC: O(n^2 * log(n)), SC: O(n^2), where n is the number of points.
pub fn min_cost_connect_points_krus(points: Vec<Vec<i32>>) -> i32 {
    let mut edges = vec![];

    for (i, x) in points.iter().enumerate() {
        for (j, y) in points.iter().skip(i).enumerate() {
            edges.push(((x[0] - y[0]).abs() + (x[1] - y[1]).abs(), i, i + j));
        }
    }

    edges.sort_unstable();

    let mut ds = DisjointSet::new(points.len());
    let mut cost = 0;
    let mut count = 0;

    for (dist, x, y) in edges {
        if ds.union(x, y) {
            cost += dist;
            count += 1;
        }

        if count == points.len() - 1 {
            break;
        }
    }

    cost
}

// Prim's algorithm with priority queue (min heap):
// 1. Start with a random point and add it to the priority queue.
// 2. While the number of included points is less than the total points:
//    - Pop the point with the smallest distance from the priority queue.
//    - Mark the point as included and add the distance to the cost.
//    - Iterate over all other points and add their distance to the priority queue.
// 3. Return the cost.
//
// TC: O(n^2 * log(n)), SC: O(n^2), where n is the number of points.
pub fn min_cost_connect_points_prim(points: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::from([Reverse((0, 0))]);
    let mut included = vec![false; points.len()];
    let mut n = 0;
    let mut cost = 0;

    while n < points.len() {
        let Reverse((dist, i)) = heap.pop().unwrap();

        if included[i] {
            continue;
        }

        included[i] = true;
        cost += dist;
        n += 1;

        for (j, point) in points.iter().enumerate() {
            heap.push(Reverse(((point[0] - points[i][0]).abs() + (point[1] - points[i][1]).abs(), j)));
        }
    }

    cost
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

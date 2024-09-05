use crate::ds::disjoint_set::DisjointSet;

// https://leetcode.com/problems/optimize-water-distribution-in-a-village
// Used Kruskal's algorithm with disjoint set to solve the problem:
// Create list of all edges between houses and wells, sort them by edge weights and iterate over them.
pub fn min_cost_to_supply_water_kruskal(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
    let mut edges = vec![];

    // Collect all edges between houses, including both directions.
    for pipe in pipes {
        edges.push(pipe); // [house1, house2, cost]
    }

    // Create one additional node a.k.a well and build edges between the well and all houses and vice versa.
    for (i, cost) in wells.iter().enumerate() {
        edges.push(vec![0, i as i32 + 1, *cost]);
    }

    edges.sort_unstable_by_key(|edge| edge[2]);

    // Use disjoint set to check if two houses are connected.
    let mut ds = DisjointSet::with_capacity(n as usize + 1);
    let mut cost = 0;

    for edge in edges {
        if ds.union(edge[0] as usize, edge[1] as usize) {
            cost += edge[2];
        }
    }

    cost
}

pub fn min_cost_to_supply_water_prims(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    let mut heap = BinaryHeap::new();

    for (i, cost) in wells.iter().enumerate() {
        heap.push(Reverse((*cost, i as i32 + 1)));
    }

    let mut seen = vec![false; n as usize + 1];
    let mut count = 0;
    let mut cost = 0;

    while count < n {
        let Reverse((dist, node)) = heap.pop().unwrap();
        if seen[node as usize] {
            continue;
        }

        seen[node as usize] = true;
        cost += dist;
        count += 1;

        for edge in &pipes {
            if edge[0] == node {
                heap.push(Reverse((edge[2], edge[1])));
            } else if edge[1] == node {
                heap.push(Reverse((edge[2], edge[0])));
            }
        }
    }

    cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_to_supply_water_kruskal() {
        let pipes = vec![vec![1, 2, 1], vec![2, 3, 1]];
        let wells = vec![1, 2, 2];
        assert_eq!(min_cost_to_supply_water_kruskal(3, wells, pipes), 3);

        let pipes = vec![vec![1, 2, 1], vec![1, 2, 2]];
        let wells = vec![1, 1];
        assert_eq!(min_cost_to_supply_water_kruskal(2, wells, pipes), 2);
    }

    #[test]
    fn test_min_cost_to_supply_water_prims() {
        let pipes = vec![vec![1, 2, 1], vec![2, 3, 1]];
        let wells = vec![1, 2, 2];
        assert_eq!(min_cost_to_supply_water_prims(3, wells, pipes), 3);

        let pipes = vec![vec![1, 2, 1], vec![1, 2, 2]];
        let wells = vec![1, 1];
        assert_eq!(min_cost_to_supply_water_prims(2, wells, pipes), 2);
    }
}

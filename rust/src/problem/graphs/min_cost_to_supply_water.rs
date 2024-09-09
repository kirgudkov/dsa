use crate::ds::disjoint_set::DisjointSet;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// https://leetcode.com/problems/optimize-water-distribution-in-a-village
// Used Kruskal's MST algorithm with disjoint set to solve the problem:
// Create list of all edges between houses and wells, sort them by edge weights and iterate over them.
pub fn min_cost_to_supply_water_kruskal(n: i32, wells: Vec<i32>, mut pipes: Vec<Vec<i32>>) -> i32 {
    // Create one additional vertex (well) and build edges between the well and all houses.
    for (i, cost) in wells.iter().enumerate() {
        pipes.push(vec![0, i as i32 + 1, *cost]);
    }

    pipes.sort_unstable_by_key(|pipe| pipe[2]);

    let mut ds = DisjointSet::with_capacity(n as usize + 1);

    pipes.iter().fold(0, |cost, pipe| {
        if ds.union(pipe[0] as usize, pipe[1] as usize) {
            cost + pipe[2]
        } else {
            cost
        }
    })
}

// Prim's MST
pub fn min_cost_to_supply_water_prims(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
    let mut heap = BinaryHeap::new();

    for (i, cost) in wells.iter().enumerate() {
        heap.push(Reverse((*cost, i as i32 + 1)));
    }

    let mut visited = vec![false; n as usize + 1];
    let mut count = 0;
    let mut min_cost = 0;

    while let Some(Reverse((cost, v))) = heap.pop() {
        if visited[v as usize] {
            continue;
        }

        visited[v as usize] = true;
        min_cost += cost;
        count += 1;

        if count == n {
            break;
        }

        pipes.iter().for_each(|pipe| {
            if pipe[0] == v {
                heap.push(Reverse((pipe[2], pipe[1])));
            } else if pipe[1] == v {
                heap.push(Reverse((pipe[2], pipe[0])));
            }
        });
    }

    min_cost
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

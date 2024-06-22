use crate::ds::disjoint_set::DisjointSet;

// https://leetcode.com/problems/find-if-path-exists-in-graph

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut ds = DisjointSet::new(n as usize);

    edges.iter().for_each(|e| {
        ds.union(e[0] as usize, e[1] as usize);
    });

    ds.find(source as usize) == ds.find(destination as usize)
}

fn valid_path_dfs(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut graph = vec![vec![]; n as usize];

    edges.iter().for_each(|e| {
        graph[e[0] as usize].push(e[1]);
        graph[e[1] as usize].push(e[0]);
    });

    let mut seen = vec![false; n as usize];
    let mut stack = vec![source];

    while let Some(v) = stack.pop() {
        seen[v as usize] = true;

        if v == destination {
            return true;
        }

        for ne in graph[v as usize].iter() {
            if !seen[*ne as usize] {
                stack.push(*ne);
            }
        }
    }

    false
}

fn valid_path_bfs(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    use std::collections::VecDeque;
    let mut graph = vec![vec![]; n as usize];

    edges.iter().for_each(|e| {
        graph[e[0] as usize].push(e[1]);
        graph[e[1] as usize].push(e[0]);
    });

    let mut q = VecDeque::from([source]);
    let mut seen = vec![false; n as usize];

    while let Some(v) = q.pop_front() {
        seen[v as usize] = true;

        if v == destination {
            return true;
        }

        for &n in graph.get(v as usize).unwrap() {
            if !seen[n as usize] {
                q.push_back(n);
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_path() {
        assert!(valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2));
        assert!(!valid_path(6, vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]], 0, 5));
    }

    #[test]
    fn test_valid_path_dfs() {
        assert!(valid_path_dfs(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2));
        assert!(!valid_path_dfs(6, vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]], 0, 5));
    }

    #[test]
    fn test_valid_path_bfs() {
        assert!(valid_path_bfs(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2));
        assert!(!valid_path_bfs(6, vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]], 0, 5));
    }
}

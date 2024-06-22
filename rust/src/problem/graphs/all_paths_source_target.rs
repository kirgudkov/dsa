pub fn all_paths_source_target_dfs_rec(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut paths = vec![];
    let mut path = vec![];

    fn dfs(v: i32, graph: &[Vec<i32>], path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>) {
        path.push(v);

        if v == graph.len() as i32 - 1 {
            paths.push(path.clone());
        }

        for n in graph.get(v as usize).unwrap() {
            dfs(*n, graph, path, paths);
            path.pop();
        }
    }

    dfs(0, &graph, &mut path, &mut paths);
    paths
}

pub fn all_paths_source_target_dfs_it(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut stack = vec![(0, vec![0])];
    let mut paths = vec![];

    while let Some((node, path)) = stack.pop() {
        if node == graph.len() as i32 - 1 {
            paths.push(path.clone());
        } else {
            for &neighbor in &graph[node as usize] {
                stack.push((neighbor, [path.clone(), vec![neighbor]].concat()));
            }
        }
    }

    paths
}

pub fn all_paths_source_target_bfs_it(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::VecDeque;

    let mut q = VecDeque::from([(0, vec![0])]);
    let mut paths = vec![];

    while let Some((v, path)) = q.pop_front() {
        if v == graph.len() as i32 - 1 {
            paths.push(path);
        } else {
            for &n in &graph[v as usize] {
                q.push_back((n, [path.clone(), vec![n]].concat()));
            }
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_paths_source_target_dfs_rec() {
        let graph = vec![
            vec![1, 2],
            vec![3],
            vec![3],
            vec![],
        ];
        let res = vec![
            vec![0, 1, 3],
            vec![0, 2, 3],
        ];
        assert_eq!(all_paths_source_target_dfs_rec(graph), res);

        let graph = vec![
            vec![4, 3, 1],
            vec![3, 2, 4],
            vec![3],
            vec![4],
            vec![],
        ];
        let res = vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ];
        assert_eq!(all_paths_source_target_dfs_rec(graph), res);
    }

    #[test]
    fn test_all_paths_source_target_dfs_it() {
        let graph = vec![
            vec![1, 2],
            vec![3],
            vec![3],
            vec![],
        ];
        let res = vec![
            vec![0, 2, 3],
            vec![0, 1, 3],
        ];
        assert_eq!(all_paths_source_target_dfs_it(graph), res);

        let graph = vec![
            vec![4, 3, 1],
            vec![3, 2, 4],
            vec![3],
            vec![4],
            vec![],
        ];
        let res = vec![
            vec![0, 1, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 3, 4],
            vec![0, 4],
        ];
        assert_eq!(all_paths_source_target_dfs_it(graph), res);
    }
    
    #[test]
    fn test_all_paths_source_target_bfs_it() {
        let graph = vec![
            vec![1, 2],
            vec![3],
            vec![3],
            vec![],
        ];
        let res = vec![
            vec![0, 1, 3],
            vec![0, 2, 3],
        ];
        assert_eq!(all_paths_source_target_bfs_it(graph), res);

        let graph = vec![
            vec![4, 3, 1],
            vec![3, 2, 4],
            vec![3],
            vec![4],
            vec![],
        ];
        let res = vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
        ];
        assert_eq!(all_paths_source_target_bfs_it(graph), res);
    }
}
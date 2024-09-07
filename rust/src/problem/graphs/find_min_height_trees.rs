// https://leetcode.com/problems/minimum-height-trees
// Editorial implementataion
fn find_min_height_trees(mut n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    if n <= 2 {
        return (0..n).collect();
    }

    let mut graph = vec![vec![]; n as usize];

    edges.iter().for_each(|edge| {
        graph[edge[0] as usize].push(edge[1]);
        graph[edge[1] as usize].push(edge[0]);
    });

    let mut leaves = graph.iter()
        .enumerate()
        .filter_map(|(vertex, neighbors)| {
            (neighbors.len() == 1).then_some(vertex as i32)
        })
        .collect::<Vec<_>>();

    while n > 2 {
        n -= leaves.len() as i32;

        leaves = leaves.iter()
            .filter_map(|&leaf| {
                let parent = graph[leaf as usize].pop()
                    .unwrap() as usize;

                graph[parent].retain(|&vertex| {
                    vertex != leaf
                });

                (graph[parent].len() == 1).then_some(parent as i32)
            })
            .collect();
    }

    leaves
}

// My implementation using indegrees
fn find_min_height_trees_2(mut n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph = vec![vec![]; n as usize];
    let mut indegrees = vec![0; n as usize];

    edges.iter().for_each(|edge| {
        graph[edge[0] as usize].push(edge[1]);
        graph[edge[1] as usize].push(edge[0]);

        indegrees[edge[0] as usize] += 1;
        indegrees[edge[1] as usize] += 1;
    });

    while n > 2 {
        let leaves = indegrees.iter()
            .enumerate()
            .filter_map(|(vertex, &indegree)| {
                (indegree == 1).then_some(vertex)
            })
            .collect::<Vec<_>>();

        n -= leaves.len() as i32;

        leaves.iter().for_each(|&leaf| {
            indegrees[leaf] = -1;

            graph[leaf].iter().for_each(|&vertex| {
                indegrees[vertex as usize] -= 1;
            });
        });
    }

    indegrees.iter()
        .enumerate()
        .filter_map(|(vertex, &indegree)| {
            matches!(indegree, 1 | 0).then_some(vertex as i32)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_height_trees() {
        let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3]];
        assert_eq!(find_min_height_trees(4, edges), vec![1]);

        let edges = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];
        assert_eq!(find_min_height_trees(6, edges), vec![3, 4]);
    }
}
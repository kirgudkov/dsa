pub fn find_min_height_trees(mut n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut adj_l = vec![vec![]; n as usize];
    let mut indegrees = vec![0i32; n as usize];

    edges.iter().for_each(|e| {
        adj_l[e[0] as usize].push(e[1]);
        adj_l[e[1] as usize].push(e[0]);

        indegrees[e[0] as usize] += 1;
        indegrees[e[1] as usize] += 1;
    });

    while n > 2 {
        let mut leaves = vec![];

        for (u, indegree) in indegrees.iter().enumerate() {
            if *indegree == 1 { leaves.push(u as i32) }
        }

        n -= leaves.len() as i32;

        for u in leaves {
            indegrees[u as usize] = -1;

            for &v in &adj_l[u as usize] {
                indegrees[v as usize] -= 1;
            }
        }
    }

    indegrees.iter()
        .enumerate()
        .filter(|&(_, &d)| d == 1 || d == 0)
        .map(|(i, _)| i as i32)
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
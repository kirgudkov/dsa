use std::collections::VecDeque;

// https://leetcode.com/problems/parallel-courses
// Kahn's Topological sort.
// The only subtle difference is that we process all leaves in nested loop, similar to "level order traversal".
// So, basically instead of sorting, we're counting levels.
// TC: O(V + E), SC: O(V + E), where V and E are the number of vertices and edges;
pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
    let mut graph = vec![vec![]; n as usize];
    let mut indegrees = vec![0; n as usize];

    relations.iter().for_each(|r| {
        graph[r[0] as usize - 1].push(r[1] as usize - 1);
        indegrees[r[1] as usize - 1] += 1;
    });

    let mut queue = indegrees.iter()
        .enumerate()
        .filter_map(|(vertex, &indegree)| {
            (indegree == 0).then_some(vertex)
        })
        .collect::<VecDeque<usize>>();

    let mut semesters = 0;
    let mut courses = 0;

    while !queue.is_empty() {
        semesters += 1;

        for _ in 0..queue.len() {
            courses += 1;

            graph[queue.pop_front().unwrap()].iter().for_each(|&next| {
                indegrees[next] -= 1;

                if indegrees[next] == 0 {
                    queue.push_back(next);
                }
            });
        }
    }

    if courses == n as usize { semesters } else { -1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_semesters() {
        let relations = vec![vec![1, 3], vec![2, 3]];
        assert_eq!(minimum_semesters(3, relations), 2);

        let relations = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(minimum_semesters(4, relations), 4);

        let relations = vec![vec![1, 2], vec![2, 3], vec![3, 1]];
        assert_eq!(minimum_semesters(3, relations), -1);

        let relations = vec![vec![1, 2], vec![1, 3], vec![2, 4]];
        assert_eq!(minimum_semesters(4, relations), 3);

        let relations = vec![vec![1, 2], vec![2, 3], vec![1, 3]];
        assert_eq!(minimum_semesters(3, relations), 3);
    }
}

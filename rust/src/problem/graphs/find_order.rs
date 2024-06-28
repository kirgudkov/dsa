use std::collections::VecDeque;

// https://leetcode.com/problems/course-schedule-ii
// Topological sort using Kahn's algorithm.
// TC: O(V + E), SC: O(V + E), where V and E are the number of vertices and edges respectively.
pub fn find_order(n: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut sorted = vec![];
    let mut adj_list = vec![vec![]; n as usize];
    let mut indegree = vec![0; n as usize];

    prerequisites.iter().for_each(|p| {
        adj_list[p[1] as usize].push(p[0]);
        indegree[p[0] as usize] += 1;
    });

    let mut q = VecDeque::new();

    indegree.iter().enumerate().for_each(|(i, &d)| {
        if d == 0 { q.push_back(i as i32) }
    });

    while let Some(course) = q.pop_front() {
        sorted.push(course);

        for &next in &adj_list[course as usize] {
            indegree[next as usize] -= 1;

            if indegree[next as usize] == 0 {
                q.push_back(next);
            }
        }
    }

    if sorted.len() == n as usize { sorted } else { vec![] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_order() {
        let prerequisites = vec![vec![1, 0]];
        assert_eq!(find_order(2, prerequisites), vec![0, 1]);

        let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        assert_eq!(find_order(4, prerequisites), vec![0, 1, 2, 3]);

        let prerequisites = vec![vec![1, 0], vec![0, 1]];
        assert_eq!(find_order(2, prerequisites), vec![]);

        let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2], vec![1, 3]];
        assert_eq!(find_order(4, prerequisites), vec![]);

        assert_eq!(find_order(1, vec![]), vec![0]);
        assert_eq!(find_order(2, vec![]), vec![0, 1]);

        let prerequisites = vec![vec![1, 0], vec![1, 2], vec![0, 1]];
        assert_eq!(find_order(3, prerequisites), vec![]);
    }
}
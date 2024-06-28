use std::collections::VecDeque;

// https://leetcode.com/problems/parallel-courses
// Topological sort using Kahn's algorithm;
// TC: O(V + E), SC: O(V + E), where V and E are the number of vertices and edges;
// 1. Represent courses as adjacency list and count indegrees.
// 2. Add all courses that do not have any prerequisites to the queue.
// 3. Increment semesters counter since we've learned all courses that were available at the moment.
// 4. For each taken course, select its next courses and decrement their indegrees and push them to the queue if their indegree becomes 0 (they've become available).
// 5. Repeat steps 3 and 4 until the queue is empty.
// 6. If we've learned all courses, return the number of semesters, otherwise return -1 (there was a cycle in the graph).
//
// So, the intuition is that each iteration (semester) we select all courses that available at the moment and learn them, 
// unblocking some of the courses that were blocked by the current ones.
pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
    let mut adj_list = vec![vec![]; n as usize];
    let mut indegrees = vec![0usize; n as usize];

    relations.iter().for_each(|r| {
        adj_list[r[0] as usize - 1].push(r[1] as usize - 1);
        indegrees[r[1] as usize - 1] += 1;
    });

    let mut q = VecDeque::new();

    indegrees.iter().enumerate().for_each(|(i, &d)| {
        if d == 0 { q.push_back(i) }
    });

    let mut semesters = 0;
    let mut courses = 0;

    while !q.is_empty() {
        semesters += 1;
        let mut n = q.len();

        while n > 0 {
            let course = q.pop_front().unwrap();
            courses += 1;

            for &next in &adj_list[course] {
                indegrees[next] -= 1;

                if indegrees[next] == 0 {
                    q.push_back(next);
                }
            }

            n -= 1;
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

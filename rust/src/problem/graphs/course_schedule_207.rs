use std::collections::VecDeque;

// Classic Topological Sort problem. Khan's algorithm is used here.
// n is the number of courses, m is the size of prerequisites.
// TC is O(V + E) due to the queue processing of V vertices and E edges
// SC is O(V + E) to store indegrees and neighbors
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    // This is a vector to store topologically sorted courses
    let mut sorted = vec![];

    // Step 1: build graph and calculate indegrees for each vertex1
    let mut graph = vec![vec![]; num_courses as usize];
    let mut indegrees = vec![0; num_courses as usize];

    prerequisites.iter().for_each(|p| {
        graph[p[1] as usize].push(p[0] as usize);
        indegrees[p[0] as usize] += 1;
    });

    // Step 2: put each vertex with indegree == 0 into the queue
    let mut queue = VecDeque::new();

    indegrees.iter().enumerate().for_each(|(vertex, &indegree)| {
        if indegree == 0 {
            queue.push_back(vertex);
        }
    });

    // Step 3: process the queue. Top item is the course we can take right now.
    while let Some(v) = queue.pop_front() {
        // So put it in the result vector
        sorted.push(v);

        // Go through every neighbour and decrement its indegree
        for &n in &graph[v] {
            indegrees[n] -= 1;

            // If neighbour's indegree becomes 0, then this course can be taken,
            // so put it in the queue
            if indegrees[n] == 0 {
                queue.push_back(n);
            }
        }
    }

    // If size is not equall then there is a cycle that prevents us from taking all courses
    sorted.len() == num_courses as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(can_finish(2, vec![vec![1, 0]]));
        assert!(!can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }
}
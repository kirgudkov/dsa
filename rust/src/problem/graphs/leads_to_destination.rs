// https://leetcode.com/problems/all-paths-from-source-lead-to-destination
// DFS with states
pub fn leads_to_destination(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut graph = vec![vec![]; n as usize];

    edges.iter().for_each(|e| {
        graph[e[0] as usize].push(e[1])
    });

    let mut stack = vec![(source, State::White)];
    let mut states = vec![State::White; n as usize];

    while let Some((node, state)) = stack.pop() {
        match state {
            State::White => {
                if states[node as usize] == State::Black {
                    continue; // Node is fully visited, skip it
                }
                if states[node as usize] == State::Gray {
                    return false; // Cycle detected
                }
                if graph[node as usize].is_empty() && node != destination {
                    return false; // Leaf node is reached and it's not the destination
                }

                states[node as usize] = State::Gray;

                // This is the most magical line;
                // Push the node back to the stack with "visiting" state 
                // to mark it as fully visited after its neighbors are visited (they are about to be pushed upon the stack)
                stack.push((node, State::Gray));

                for &neighbor in &graph[node as usize] {
                    stack.push((neighbor, State::White));
                }
            }
            State::Gray => {
                states[node as usize] = State::Black;
            }
            _ => unreachable!(),
        }
    }

    true
}

#[derive(Clone, PartialEq)]
enum State {
    White, // Unvisited
    Gray, // Visiting
    Black, // Fully visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]];
        assert!(leads_to_destination(4, edges, 0, 3));

        let edges = vec![vec![0, 1], vec![0, 2]];
        assert!(!leads_to_destination(3, edges, 0, 2));

        let edges = vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![2, 1]];
        assert!(!leads_to_destination(4, edges, 0, 3));
    }
}
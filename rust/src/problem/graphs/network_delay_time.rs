use std::cmp::Reverse;
use std::collections::BinaryHeap;

// https://leetcode.com/problems/network-delay-time
// Dijkstra's algorithm.
// TC: O(N + E * log(N))
pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let mut minimums = vec![i32::MAX; n as usize];
    minimums[k as usize - 1] = 0;

    let mut adj = vec![vec![]; n as usize];
    for edge in &times {
        adj[edge[0] as usize - 1].push((edge[1], edge[2]))
    }

    let mut pq = BinaryHeap::from([Reverse((0, k))]); // (time, node)
    let mut seen = vec![false; n as usize];

    while let Some(Reverse((curr_time, curr_node))) = pq.pop() {
        // Skip if result already contains time that is less than the current one or if the node is already seen.
        if curr_time > minimums[curr_node as usize - 1] || seen[curr_node as usize - 1] {
            continue;
        }

        seen[curr_node as usize - 1] = true;

        for &(nei, nei_time) in &adj[curr_node as usize - 1] {
            // Edge relaxation:
            // If the time to reach the current node + the time to reach the neighbor is
            // less than the previously discovered time to reach the neighbor,
            // update the time to reach the neighbor and add it to the priority queue.
            //          "1"
            //       1 /   \ 3
            //      "2" --- "3"
            //           1
            // Previously discovered time to reach 3  is 3;
            // Now, we are at the node 2 and the time to reach 2 is 1 + the time to reach 3 from 2 is 1 = 1 + 1 = 2
            // 2 < 3, so we update the time to reach 3 to 2.
            if minimums[curr_node as usize - 1] + nei_time < minimums[nei as usize - 1] {
                minimums[nei as usize - 1] = minimums[curr_node as usize - 1] + nei_time;
                pq.push(Reverse((minimums[nei as usize - 1], nei)));
            }
        }
    }

    minimums.sort_unstable();

    if minimums[n as usize - 1] == i32::MAX {
        -1
    } else {
        minimums[n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_delay_time() {
        let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
        assert_eq!(network_delay_time(times, 4, 2), 2);

        let times = vec![vec![1, 2, 1]];
        assert_eq!(network_delay_time(times, 2, 1), 1);

        let times = vec![vec![1, 2, 1]];
        assert_eq!(network_delay_time(times, 2, 2), -1);

        let times = vec![vec![1, 2, 1], vec![2, 3, 2], vec![1, 3, 4]];
        assert_eq!(network_delay_time(times, 3, 1), 3);
    }
}
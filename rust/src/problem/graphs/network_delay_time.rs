use std::cmp::Reverse;
use std::collections::BinaryHeap;

// https://leetcode.com/problems/network-delay-time
// Dijkstra's algorithm.
// TC: O(N + E * log(N))
pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize - 1;

    let graph = times.iter().fold(vec![vec![]; n], |mut acc, edge| {
        acc[edge[0] as usize - 1].push((edge[1] as usize - 1, edge[2] as usize));
        acc
    });

    let mut min_time = vec![usize::MAX; n];
    min_time[k] = 0;

    let mut heap = BinaryHeap::from([Reverse((0, k))]);
    let mut visited = vec![false; n];

    while let Some(Reverse((time, v))) = heap.pop() {
        if visited[v] || time > min_time[v] {
            continue;
        }

        visited[v] = true;

        graph[v].iter().for_each(|&(u, u_time)| {
            // Edge relaxation:
            // If the time to reach the current vertex + time to reach the neighbor is
            // less than the previously discovered time to reach the neighbor,
            // update the time to reach the neighbor and add it to the priority queue.
            //          "1"
            //       1 /   \ 3
            //      "2" --- "3"
            //           1
            // Previously discovered time to reach 3  is 3;
            // Now, we are at the node 2 and the time to reach 2 is 1 + the time to reach 3 from 2 is 1 = 1 + 1 = 2
            // 2 < 3, so we update the time to reach 3 to 2.
            if min_time[v] + u_time < min_time[u] {
                min_time[u] = min_time[v] + u_time;
                heap.push(Reverse((min_time[u], u)));
            }
        });
    }

    min_time.sort_unstable();

    if min_time[n - 1] == usize::MAX {
        -1
    } else {
        min_time[n - 1] as i32
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
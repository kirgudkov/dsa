// https://leetcode.com/problems/cheapest-flights-within-k-stops/
pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, mut k: i32) -> i32 {
    use std::collections::VecDeque;

    let mut adj = vec![vec![]; n as usize];
    for flight in &flights {
        adj[flight[0] as usize].push((flight[1], flight[2]));
    }

    let mut q = VecDeque::from([(src, 0)]);
    let mut costs = vec![i32::MAX; n as usize];
    costs[src as usize] = 0;

    while k >= 0 && !q.is_empty() {
        let mut size = q.len();

        while size > 0 {
            let (node, cost) = q.pop_front().unwrap();

            for &(nei, nei_cost) in &adj[node as usize] {
                if cost + nei_cost < costs[nei as usize] {
                    costs[nei as usize] = cost + nei_cost;
                    q.push_back((nei, costs[nei as usize]));
                }
            }

            size -= 1;
        }

        k -= 1;
    }

    if costs[dst as usize] == i32::MAX { -1 } else { costs[dst as usize] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_cheapest_price() {
        let flights = vec![vec![0, 1, 1], vec![0, 2, 5], vec![1, 2, 1], vec![2, 3, 1], vec![3, 4, 1]];
        assert_eq!(find_cheapest_price(5, flights, 0, 4, 2), 7);

        let flights = vec![vec![0, 1, 1], vec![0, 2, 5], vec![1, 2, 1], vec![2, 3, 1]];
        assert_eq!(find_cheapest_price(4, flights, 0, 3, 1), 6);

        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![2, 0, 100], vec![1, 3, 600], vec![2, 3, 200]];
        assert_eq!(find_cheapest_price(4, flights, 0, 3, 1), 700);

        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        assert_eq!(find_cheapest_price(3, flights, 0, 2, 1), 200);

        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        assert_eq!(find_cheapest_price(3, flights, 0, 2, 0), 500);
    }
}
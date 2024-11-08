use std::cmp::Reverse;
use std::collections::BinaryHeap;

// https://leetcode.com/problems/minimum-cost-to-connect-sticks
pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::from(
        sticks.iter()
            .map(|&stick| Reverse(stick))
            .collect::<Vec<_>>()
    );

    let mut cost = 0;

    while heap.len() > 1 {
        let a = heap.pop().unwrap().0;
        let b = heap.pop().unwrap().0;

        cost += a + b;

        heap.push(Reverse(a + b));
    }

    cost
}

// Alternative way. Probably could be the first version that leads to optimized heap solution;
// Each recursuive call reduces sticks count by 1. So it runs in O(n) time, each recursive call, we sort sticks in O(n log n)
// So, the overall time complexity is O(n^2 log n)
pub fn connect_sticks_rec(mut sticks: Vec<i32>, cost: i32) -> i32 {
    if sticks.len() == 1 { // We have one stick; All sticks were connected
        return cost;
    }

    sticks.sort_unstable_by(|a, b| b.cmp(a));

    let a = sticks.pop().unwrap();
    let b = sticks.pop().unwrap();
    sticks.push(a + b);

    connect_sticks_rec(sticks, cost + a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_sticks() {
        let sticks = vec![2, 4, 3];
        assert_eq!(connect_sticks(sticks), 14);

        let sticks = vec![1, 8, 3, 5];
        assert_eq!(connect_sticks(sticks), 30);
    }

    #[test]
    fn test_connect_sticks_rec() {
        let sticks = vec![2, 4, 3];
        assert_eq!(connect_sticks_rec(sticks, 0), 14);

        let sticks = vec![1, 8, 3, 5];
        assert_eq!(connect_sticks_rec(sticks, 0), 30);
    }
}
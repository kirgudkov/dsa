use std::cmp::Reverse;
use std::collections::BinaryHeap;

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
}
use std::collections::VecDeque;

// https://leetcode.com/problems/cheapest-flights-within-k-stops/
pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, mut stops: i32) -> i32 {
    let graph = flights.iter().fold(vec![vec![]; n as usize], |mut acc, flight| {
        acc[flight[0] as usize].push((flight[1], flight[2]));
        acc
    });

    let mut queue = VecDeque::from([(src, 0)]);
    let mut prices = vec![i32::MAX; n as usize];

    while stops >= 0 && !queue.is_empty() {
        for _ in 0..queue.len() {
            let (v, price) = queue.pop_front().unwrap();

            graph[v as usize].iter().for_each(|&(n, n_price)| {
                if price + n_price < prices[n as usize] {
                    prices[n as usize] = price + n_price;
                    queue.push_back((n, prices[n as usize]));
                }
            });
        }

        stops -= 1;
    }

    if prices[dst as usize] == i32::MAX { -1 } else { prices[dst as usize] }
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
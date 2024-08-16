// https://leetcode.com/problems/last-stone-weight
// TC is O(n log n)
// SC is O(n)
pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap = std::collections::BinaryHeap::from(stones);

    while heap.len() > 1 {
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();

        if a != b {
            heap.push(a - b);
        }
    }

    *heap.peek().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_stone_weight() {
        assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
        assert_eq!(last_stone_weight(vec![1, 3]), 2);
        assert_eq!(last_stone_weight(vec![2, 2]), 0);
    }
}
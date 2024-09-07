use std::collections::{BinaryHeap, HashMap};

// https://leetcode.com/problems/top-k-frequent-elements

// The most naive approach: count frequency of each element, sort map by frequency and take first k elements. 
// TC: O(n log n)
// SC: O(n)
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq = HashMap::new();

    for num in nums.iter() {
        *freq.entry(num).or_insert(0) += 1;
    }

    // collect into a vector of tuples (item, frequency)
    let mut freq_vec: Vec<(&i32, i32)> = freq.into_iter().collect();
    // sort by frequency in a descending order
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // leverage rust iterator to take first k elements
    freq_vec.iter()
        .take(k as usize)
        .map(|(num, _)| **num)
        .collect()
}

// Slightly better approach: use max heap to keep track of k most frequent elements
// TC: O(n log n)
// SC: O(n)
pub fn top_k_frequent_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq = HashMap::new();
    let mut heap = BinaryHeap::new();

    for num in nums.iter() {
        *freq.entry(num).or_insert(0) += 1;
    }

    for (num, count) in freq.iter() {
        heap.push((count, num));
    }

    let mut result = vec![];

    for _ in 0..k {
        result.push(**heap.pop().unwrap().1);
    }

    result
}

// The beast approach: using bucket sort
// TC: O(n)
// SC: O(n)
fn top_k_frequent_bucket_sort(input: &[i32], k: i32) -> Vec<i32> {
    let mut buckets: Vec<Vec<i32>> = vec![vec![]; input.len() + 1];
    let mut freq = HashMap::new();

    for num in input.iter() {
        *freq.entry(num).or_insert(0) += 1;
    }

    for (num, count) in freq.iter() {
        buckets[*count as usize].push(**num);
    }

    let mut result = vec![];

    for elem in buckets.iter().rev() {
        for num in elem.iter() {
            result.push(*num);

            if result.len() == k as usize {
                return result;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::problem::misc::top_k_frequent::{top_k_frequent, top_k_frequent_bucket_sort, top_k_frequent_heap};

    #[test]
    fn test_top_k_frequent_heap() {
        assert_eq!(top_k_frequent_heap(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent_heap(vec![1], 1), vec![1]);
    }

    #[test]
    fn test_top_k_frequent_bucket_sort() {
        assert_eq!(top_k_frequent_bucket_sort(&[1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent_bucket_sort(&[1], 1), vec![1]);
    }

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}

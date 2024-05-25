use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freqs = HashMap::new();

    for num in nums.iter() {
        let count = freqs.entry(num).or_insert(0);
        *count += 1;
    }

    let mut freqs_vec: Vec<(&i32, i32)> = freqs.into_iter().collect();
    freqs_vec.sort_by(|a, b| b.1.cmp(&a.1));
    freqs_vec
        .iter()
        .take(k as usize)
        .map(|(num, _)| **num)
        .collect()
}

fn bucket_sort(input: &mut [i32], k: i32) {
    let max = *input.iter().max().unwrap();
    let mut buckets: Vec<Vec<i32>> = vec![vec![]; k as usize];

    for item in input.iter() {
        let index = (*item * k / max) as usize;
        if index == k as usize {
            buckets[k as usize - 1].push(*item);
        } else {
            buckets[index].push(*item);
        }
    }

    buckets
        .iter_mut()
        .for_each(|item| item.sort_unstable());
    input.copy_from_slice(buckets.concat().as_slice());
}

#[cfg(test)]
mod tests {
    use crate::problem::top_k_frequent::{bucket_sort, top_k_frequent};

    #[test]
    fn test_bucket_sort() {
        let mut input = vec![3, 2, 1, 4, 5, 6, 7, 8, 9, 10];
        bucket_sort(&mut input, 10);
        assert_eq!(input, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut input = vec![3, 2, 1, 4, 5, 6, 7, 8, 9, 10];
        bucket_sort(&mut input, 5);
        assert_eq!(input, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut input = vec![3, 2, 1, 4, 5, 6, 7, 8, 9, 10];
        bucket_sort(&mut input, 3);
        assert_eq!(input, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut input = vec![1];
        bucket_sort(&mut input, 1);
        assert_eq!(input, vec![1]);
    }

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}

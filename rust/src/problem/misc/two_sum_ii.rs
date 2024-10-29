use std::cmp::Ordering::{Equal, Greater, Less};
// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted
// Two pointers approach;
// TC is O(n) in the worst case. But binary search could improve performance in some cases;
// SC is O(1);
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    // Works similar to built in binary_search: 
    // - returns Ok(usize) if target was found, contains index of the element;
    // - returns Err(usize) if target wasn't found, contains index of insert position;
    let binary_search = |target: &i32| -> Result<usize, usize> {
        let mut l = 0;
        let mut r = numbers.len();

        while l < r {
            let m = (l + r) / 2;

            match numbers[m].cmp(target) {
                Less => l = m + 1,
                Greater => r = m,
                Equal => return Ok(m),
            }
        }

        Err(l)
    };

    let mut l = 0;
    // Here is the main difference with the pure two pointers approach;
    // Originally we would have set r as numbers.len() - 1;
    let mut r = binary_search(&(target - numbers[0]))
        .unwrap_or_else(|x| x - 1);

    loop {
        if numbers[l] + numbers[r] == target {
            return vec![l as i32 + 1, r as i32 + 1];
        }

        if numbers[l] + numbers[r] > target {
            r -= 1;
        } else {
            l += 1;
        }
    }
}

// Hash map approach;
// TC is O(n); 
// SC is O(n);
pub fn two_sum_hm(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = std::collections::HashMap::new();

    for (i, &num) in numbers.iter().enumerate() {
        if let Some(&j) = hm.get(&(target - num)) {
            return vec![j + 1, i as i32 + 1];
        }

        hm.insert(num, i as i32);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![12, 13, 23, 28, 43, 44, 59, 60, 61, 68, 70, 86, 88, 92, 124, 125, 136, 168, 173, 173, 180, 199, 212, 221, 227, 230, 277, 282, 306, 314, 316, 321, 325, 328, 336, 337, 363, 365, 368, 370, 370, 371, 375, 384, 387, 394, 400, 404, 414, 422, 422, 427, 430, 435, 457, 493, 506, 527, 531, 538, 541, 546, 568, 583, 585, 587, 650, 652, 677, 691, 730, 737, 740, 751, 755, 764, 778, 783, 785, 789, 794, 803, 809, 815, 847, 858, 863, 863, 874, 887, 896, 916, 920, 926, 927, 930, 933, 957, 981, 997], 542), vec![24, 32]);
        assert_eq!(two_sum(vec![3, 24, 50, 79, 88, 150, 345], 200), vec![3, 6]);
        assert_eq!(two_sum(vec![0, 0, 3, 4], 0), vec![1, 2]);
        assert_eq!(two_sum(vec![5, 25, 75], 100), vec![2, 3]);
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(two_sum(vec![-1, 0], -1), vec![1, 2]);
        assert_eq!(two_sum(vec![-1, -1, 1, 1, 1], -2), vec![1, 2]);
    }

    #[test]
    fn test_two_sum_hm() {
        assert_eq!(two_sum_hm(vec![0, 0, 3, 4], 0), vec![1, 2]);
        assert_eq!(two_sum_hm(vec![5, 25, 75], 100), vec![2, 3]);
        assert_eq!(two_sum_hm(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(two_sum_hm(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(two_sum_hm(vec![-1, 0], -1), vec![1, 2]);
        assert_eq!(two_sum_hm(vec![-1, -1, 1, 1, 1], -2), vec![1, 2]);
    }
}
// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted
// Two pointers approach;
// TC is O(n) in the worst case. But binary search could improve performance in some cases;
// SC is O(1);
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let bs = || -> usize {
        let mut l = 0;
        let mut r = numbers.len() - 1;

        while l < r {
            let m = l + (r - l) / 2;

            if numbers[m] <= target - numbers[0] {
                l = m + 1;
            } else {
                r = m;
            }
        }

        l
    };

    let mut l = 0;

    // Here is the main difference with pure two pointers approach;
    // Originally we would have set r as numbers.len() - 1;
    let mut r = bs();

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

    for (i, num) in numbers.iter().enumerate() {
        if hm.contains_key(&(target - num)) {
            return vec![*hm.get(&(target - num)).unwrap() + 1, i as i32 + 1];
        }

        hm.insert(*num, i as i32);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(numbers, target), vec![1, 2]);

        let numbers = vec![2, 3, 4];
        let target = 6;
        assert_eq!(two_sum(numbers, target), vec![1, 3]);

        let numbers = vec![-1, 0];
        let target = -1;
        assert_eq!(two_sum(numbers, target), vec![1, 2]);

        let numbers = vec![0, 0, 3, 4];
        let target = 0;
        assert_eq!(two_sum(numbers, target), vec![1, 2]);

        let numbers = vec![-1, -1, 1, 1, 1];
        let target = -2;
        assert_eq!(two_sum(numbers, target), vec![1, 2]);
    }

    #[test]
    fn test_two_sum_hm() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum_hm(numbers, target), vec![1, 2]);

        let numbers = vec![2, 3, 4];
        let target = 6;
        assert_eq!(two_sum_hm(numbers, target), vec![1, 3]);

        let numbers = vec![-1, 0];
        let target = -1;
        assert_eq!(two_sum_hm(numbers, target), vec![1, 2]);

        let numbers = vec![0, 0, 3, 4];
        let target = 0;
        assert_eq!(two_sum_hm(numbers, target), vec![1, 2]);

        let numbers = vec![-1, -1, 1, 1, 1];
        let target = -2;
        assert_eq!(two_sum_hm(numbers, target), vec![1, 2]);
    }
}
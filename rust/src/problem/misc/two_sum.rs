// T: O(n^2), S: O(1)
fn two_sum_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, &a) in nums.iter().enumerate() {
        for (j, &b) in nums.iter().enumerate().skip(i + 1) {
            if a + b == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    unreachable!()
}
// T: O(n) two-pass, S: O(n)
fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index_map = std::collections::HashMap::<i32, usize>::new();

    for (i, &num) in nums.iter().enumerate() {
        index_map.insert(num, i);
    }

    for (i, &num) in nums.iter().enumerate() {
        if index_map.contains_key(&(target - num)) && index_map[&(target - num)] != i {
            return vec![i as i32, index_map[&(target - num)] as i32];
        }
    }

    unreachable!()
}
// T: O(n) single-pass, S: O(n)
fn two_sum_3(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index_map = std::collections::HashMap::<i32, usize>::new();

    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = index_map.get(&(target - num)) {
            return vec![j as i32, i as i32];
        }

        index_map.insert(num, i);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(super::two_sum_1(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(super::two_sum_1(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(super::two_sum_1(vec![3, 3], 6), vec![0, 1]);

        assert_eq!(super::two_sum_2(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(super::two_sum_2(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(super::two_sum_2(vec![3, 3], 6), vec![0, 1]);

        assert_eq!(super::two_sum_3(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(super::two_sum_3(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(super::two_sum_3(vec![3, 3], 6), vec![0, 1]);
    }
}

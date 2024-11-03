use std::collections::{HashSet, VecDeque};

pub fn jump(nums: Vec<i32>) -> i32 {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([(0, 0)]);

    while let Some((i, steps)) = queue.pop_front() {
        if i == nums.len() - 1 {
            return steps;
        }

        if seen.contains(&i) {
            continue;
        }

        seen.insert(i);

        for delta in 1..=nums[i] as usize {
            queue.push_back((i + delta, steps + 1));
        }
    }

    unreachable!()
}

pub fn jump_greedy(nums: Vec<i32>) -> i32 {
    let mut a = 0;
    let mut b = 0;
    let mut jumps = 0;

    for i in 0..nums.len()-1 {
        b = b.max(i + nums[i] as usize);

        if i == a {
            jumps += 1;
            a = b;
        }
    }
    
    jumps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(jump(vec![5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0]), 3);
        assert_eq!(jump(vec![1, 2, 1, 1, 1]), 3);
        assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
        assert_eq!(jump(vec![0]), 0);
        assert_eq!(jump(vec![1, 2]), 1);
        assert_eq!(jump(vec![2, 1]), 1);

        assert_eq!(jump_greedy(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(jump_greedy(vec![5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0]), 3);
        assert_eq!(jump_greedy(vec![1, 2, 1, 1, 1]), 3);
        assert_eq!(jump_greedy(vec![2, 3, 0, 1, 4]), 2);
        assert_eq!(jump_greedy(vec![0]), 0);
        assert_eq!(jump_greedy(vec![1, 2]), 1);
        assert_eq!(jump_greedy(vec![2, 1]), 1);
    }
}
// https://leetcode.com/problems/two-sum-iii-data-structure-design
// Two pointers approach;
// TC is O(n * log n) in the worst case (which should appear quite often) 
// and O(n) in the best case, when the array is already sorted;
// SC is O(n);
struct TwoSum {
    sorted: bool,
    vec: Vec<i32>,
}

impl TwoSum {
    fn new() -> Self {
        Self {
            vec: vec![],
            sorted: false,
        }
    }

    fn add(&mut self, number: i32) {
        self.vec.push(number);
        self.sorted = false;
    }

    fn find(&mut self, value: i32) -> bool {
        if self.vec.len() < 2 {
            return false;
        }

        if !self.sorted {
            self.vec.sort_unstable();
            self.sorted = true;
        }

        let mut l = 0;
        let mut r = self.vec.len() - 1;

        // Binary search won't make a big difference 
        // because we still would have to traverse n elements in the worst case (when there is no pair that sums to target)
        while l < r {
            let sum = self.vec[l] + self.vec[r];

            if sum == value {
                return true;
            }

            if sum > value {
                r -= 1;
            } else {
                l += 1;
            }
        }

        false
    }
}

// Hash map approach;
// TC is O(n)
// SC is O(n);
struct TwoSumHM {
    map: std::collections::HashMap<i32, i32>,
}

impl TwoSumHM {
    fn new() -> Self {
        Self {
            map: std::collections::HashMap::new()
        }
    }

    fn add(&mut self, number: i32) {
        *self.map.entry(number).or_insert(0) += 1;
    }

    fn find(&self, target: i32) -> bool {
        for &a in self.map.keys() {
            let b = target - a;

            if b == a {
                if *self.map.get(&a).unwrap() > 1 {
                    return true;
                }
            } else if self.map.contains_key(&b) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tp() {
        let mut ts = TwoSum::new();
        assert!(!ts.find(0));
        ts.add(1);
        assert!(!ts.find(1));
        ts.add(3);
        ts.add(5);
        assert!(ts.find(4));
        assert!(!ts.find(7));

        let mut ts = TwoSum::new();
        ts.add(3);
        ts.add(1);
        ts.add(2);
        assert!(ts.find(3));
    }

    #[test]
    fn test_hm() {
        let mut ts = TwoSumHM::new();
        assert!(!ts.find(0));
        ts.add(1);
        assert!(!ts.find(1));
        ts.add(3);
        ts.add(5);
        assert!(ts.find(4));
        assert!(!ts.find(7));

        let mut ts = TwoSumHM::new();
        ts.add(3);
        ts.add(1);
        ts.add(2);
        assert!(ts.find(3));
    }
}
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// https://leetcode.com/problems/find-median-from-data-stream/description/
// TC: O(log n) for binary search + O(n) for insert
// SC: O(n)
struct MedianFinder {
    vec: Vec<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Self { vec: vec![] }
    }

    fn add_num(&mut self, num: i32) {
        if self.vec.is_empty() {
            self.vec.push(num);
        } else {
            let mut l = 0i32;
            let mut r = (self.vec.len() - 1) as i32;

            while l <= r {
                let m = l + (r - l) / 2;

                match self.vec[m as usize].cmp(&num) {
                    std::cmp::Ordering::Less => l = m + 1,
                    std::cmp::Ordering::Greater => r = m - 1,
                    std::cmp::Ordering::Equal => {
                        self.vec.insert(m as usize, num);
                        return;
                    }
                }
            }

            self.vec.insert(l as usize, num);
        }
    }

    fn find_median(&self) -> f64 {
        if self.vec.len() % 2 == 0 {
            (self.vec[self.vec.len() / 2] + self.vec[self.vec.len() / 2 - 1]) as f64 / 2.0
        } else {
            self.vec[self.vec.len() / 2] as f64
        }
    }
}

// TC: O(log n)
// SC: O(n)
struct MedianFinder2 {
    left_heap: BinaryHeap<i32>,
    right_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder2 {
    fn new() -> Self {
        Self {
            left_heap: BinaryHeap::new(),
            right_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.left_heap.push(num);
        self.right_heap.push(Reverse(self.left_heap.pop().unwrap()));

        if self.right_heap.len() > self.left_heap.len() {
            self.left_heap.push(self.right_heap.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        if self.left_heap.len() > self.right_heap.len() {
            *self.left_heap.peek().unwrap() as f64
        } else {
            (self.left_heap.peek().unwrap() + self.right_heap.peek().unwrap().0) as f64 / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_finder() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        assert_eq!(mf.find_median(), 1.0);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);

        let mut mf = MedianFinder::new();
        mf.add_num(-1);
        assert_eq!(mf.find_median(), -1.0);
        mf.add_num(-2);
        assert_eq!(mf.find_median(), -1.5);
        mf.add_num(-3);
        assert_eq!(mf.find_median(), -2.0);
        mf.add_num(-4);
        assert_eq!(mf.find_median(), -2.5);
        mf.add_num(-5);
        assert_eq!(mf.find_median(), -3.0);
    }

    #[test]
    fn test_median_finder2() {
        let mut mf = MedianFinder2::new();
        mf.add_num(1);
        assert_eq!(mf.find_median(), 1.0);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);

        let mut mf = MedianFinder2::new();
        mf.add_num(-1);
        assert_eq!(mf.find_median(), -1.0);
        mf.add_num(-2);
        assert_eq!(mf.find_median(), -1.5);
        mf.add_num(-3);
        assert_eq!(mf.find_median(), -2.0);
        mf.add_num(-4);
        assert_eq!(mf.find_median(), -2.5);
        mf.add_num(-5);
        assert_eq!(mf.find_median(), -3.0);
    }
}
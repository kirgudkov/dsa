use std::cmp::Reverse;
use std::collections::BinaryHeap;

// https://leetcode.com/problems/find-median-from-data-stream/description/
// TC: O(log n) for binary search + O(n) for insert
// SC: O(n)
#[derive(Default)]
struct MedianFinder {
    vec: Vec<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, num: i32) {
        let mut l = 0;
        let mut r = self.vec.len();

        while l < r {
            let m = l + (r - l) / 2;

            if self.vec[m] == num {
                self.vec.insert(m, num);
                return;
            }

            if self.vec[m] > num {
                r = m;
            } else {
                l = m + 1;
            }
        }

        self.vec.insert(l, num);
    }

    fn find_median(&self) -> f64 {
        if self.vec.len() % 2 == 0 {
            (self.vec[self.vec.len() / 2] + self.vec[self.vec.len() / 2 - 1]) as f64 / 2.0
        } else {
            self.vec[self.vec.len() / 2] as f64
        }
    }
}

// Two heaps approach. Technically, it doesn't comply with problem description:
// ...The median is the middle value in an **ordered** integer list...
// But it does the job, it finds median.
//
// TC: O(log n)
// SC: O(n)
#[derive(Default)]
struct MedianFinder2 {
    left_heap: BinaryHeap<i32>,
    right_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder2 {
    fn new() -> Self {
        Default::default()
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
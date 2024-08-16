use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut instance = Self {
            heap: BinaryHeap::new(),
            k: k as usize,
        };

        for &num in nums.iter() {
            instance.add(num);
        }

        instance
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() == self.k {
            if val > self.heap.peek().unwrap().0 {
                self.heap.pop();
                self.heap.push(Reverse(val));
            }
        } else {
            self.heap.push(Reverse(val));
        }

        self.heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_largest() {
        let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth_largest.add(3), 4);
        assert_eq!(kth_largest.add(5), 5);
        assert_eq!(kth_largest.add(10), 5);
        assert_eq!(kth_largest.add(9), 8);
        assert_eq!(kth_largest.add(4), 8);
    }
}

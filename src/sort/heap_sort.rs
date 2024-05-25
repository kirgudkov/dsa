pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut heap = MinHeap::from(nums);

    while let Some(min) = heap.pop() {
        result.push(min);
    }

    result
}

struct MinHeap {
    vec: Vec<i32>,
}

impl MinHeap {
    pub fn from(vec: Vec<i32>) -> Self {
        let mut instance = Self { vec: vec![] };

        for item in vec.iter() {
            instance.insert(*item);
        }

        instance
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.vec.is_empty() {
            return None;
        }

        let last_idx = self.vec.len() - 1;
        self.vec.swap(0, last_idx);

        let last_value = self.vec.pop();

        if self.vec.len() < 2 {
            return last_value;
        }

        // Maintain the heap property from the top down
        let mut i = 0;
        while let Some((left, right)) = self.get_children(i) {
            let idx = match (left, right) {
                (None, None) => break,
                (Some(left_idx), None) => left_idx,
                (None, Some(right_idx)) => right_idx,
                (Some(left_idx), Some(right_idx)) => {
                    let condition = self.vec[left_idx] < self.vec[right_idx];

                    if condition { left_idx } else { right_idx }
                }
            };

            if self.vec[i] > self.vec[idx] {
                self.vec.swap(i, idx);
                i = idx;
            } else {
                break;
            }
        }

        last_value
    }

    fn insert(&mut self, value: i32) {
        self.vec.push(value);

        if self.vec.is_empty() {
            return;
        }

        // Maintain the heap property from the bottom up
        let mut index = self.vec.len() - 1;

        while let Some(parent_idx) = self.get_parent(index) {
            if self.vec[index] < self.vec[parent_idx] {
                self.vec.swap(index, parent_idx);
                index = parent_idx;
            } else {
                break;
            }
        }
    }

    fn get_parent(&self, index: usize) -> Option<usize> {
        let _ = self.vec.get(index.checked_sub(1)? / 2)?;

        Some((index - 1) / 2)
    }

    fn get_children(&self, i: usize) -> Option<(Option<usize>, Option<usize>)> {
        let l = self.vec.get((i * 2) + 1).map(|_| (i * 2) + 1);
        let r = self.vec.get((i * 2) + 2).map(|_| (i * 2) + 2);

        Some((l, r))
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::heap_sort::sort_array;

    #[test]
    fn test_sort_array() {
        assert_eq!(sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
        assert_eq!(sort_array(vec![5, 1, 1, 2, 0, 0]), vec![0, 0, 1, 1, 2, 5]);
    }
}

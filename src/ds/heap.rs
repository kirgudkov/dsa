enum HeapType {
    Max,
    Min,
}

pub struct Heap {
    list: Vec<i32>,
    heap_type: HeapType,
}

impl std::fmt::Debug for Heap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.list)
    }
}

impl Heap {
    fn create(list: Vec<i32>, heap_type: HeapType) -> Self {
        let mut instance = Self { list, heap_type };
        instance.heapify();
        instance
    }

    pub fn max(list: Vec<i32>) -> Self {
        Self::create(list, HeapType::Max)
    }

    pub fn min(list: Vec<i32>) -> Self {
        Self::create(list, HeapType::Min)
    }

    pub fn as_sorted(&mut self) -> Vec<i32> {
        let mut result = Vec::new();

        while let Some(value) = self.pop() {
            result.push(value);
        }

        result
    }

    pub fn insert(&mut self, value: i32) {
        self.list.push(value);
        self.maintain_up();
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.list.is_empty() {
            return None;
        }

        let a = 0;
        let b = self.list.len() - 1;
        self.list.swap(a, b);

        let last = self.list.pop();
        self.maintain_down();

        last
    }

    fn heapify(&mut self) {
        let mut i = self.list.len() / 2;
        while i > 0 {
            i -= 1;
            self.maintain_down();
        }
    }

    fn maintain_down(&mut self) {
        if self.list.len() < 2 {
            return;
        }

        let mut i = 0;
        while let Some((left_idx, right_idx)) = self.get_kids(i) {
            let idx = match (left_idx, right_idx) {
                (None, None) => break,
                (Some(left_idx), None) => left_idx,
                (None, Some(right_idx)) => right_idx,
                (Some(left_idx), Some(right_idx)) => {
                    let condition = match self.heap_type {
                        HeapType::Max => self.list[left_idx] > self.list[right_idx],
                        HeapType::Min => self.list[left_idx] < self.list[right_idx],
                    };

                    if condition {
                        left_idx
                    } else {
                        right_idx
                    }
                }
            };

            let condition = match self.heap_type {
                HeapType::Max => self.list[i] < self.list[idx],
                HeapType::Min => self.list[i] > self.list[idx],
            };

            if condition {
                self.list.swap(i, idx);
                i = idx;
            } else {
                break;
            }
        }
    }

    fn maintain_up(&mut self) {
        if self.list.is_empty() {
            return;
        }

        let mut i = self.list.len() - 1;

        while let Some(parent_idx) = self.get_parent(i) {
            let condition = match self.heap_type {
                HeapType::Max => self.list[i] > self.list[parent_idx],
                HeapType::Min => self.list[i] < self.list[parent_idx],
            };

            if condition {
                self.list.swap(i, parent_idx);
                i = parent_idx;
            } else {
                break;
            }
        }
    }

    fn get_kids(&self, i: usize) -> Option<(Option<usize>, Option<usize>)> {
        let l = self.list.get((i * 2) + 1).map(|_| (i * 2) + 1);
        let r = self.list.get((i * 2) + 2).map(|_| (i * 2) + 2);

        Some((l, r))
    }

    fn get_parent(&self, index: usize) -> Option<usize> {
        let _ = self.list.get(index.checked_sub(1)? / 2)?;

        Some((index - 1) / 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::ds::heap::Heap;

    #[test]
    fn test_max_heap() {
        let mut max_heap = Heap::max(vec![12, 14, 20, 9, 10, 30, 18]);
        assert_eq!(max_heap.as_sorted(), vec![30, 20, 18, 14, 12, 10, 9]);
    }
}

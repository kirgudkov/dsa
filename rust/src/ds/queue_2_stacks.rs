#[derive(Default)]
struct MyQueue {
    val_vec: Vec<i32>,
    idx_vec: Vec<usize>,
}

impl MyQueue {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, x: i32) {
        self.val_vec.push(x);

        if self.idx_vec.is_empty() {
            self.idx_vec.push(0);
        }
    }

    fn pop(&mut self) -> i32 {
        let i = self.idx_vec.pop().unwrap();
        let top = self.val_vec[i];

        if i == self.val_vec.len() - 1 {
            self.idx_vec = Vec::new();
            self.val_vec = Vec::new();
        } else {
            self.idx_vec.push(i + 1);
        }

        top
    }

    fn peek(&self) -> i32 {
        self.val_vec[*self.idx_vec.last().unwrap()]
    }

    fn empty(&self) -> bool {
        self.idx_vec.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::ds::queue_2_stacks::MyQueue;

    #[test]
    fn test_queue() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert!(!queue.empty());
    }
}
struct MyQueue {
    v_stack: Vec<i32>,
    i_stack: Vec<usize>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            v_stack: Vec::new(),
            i_stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.v_stack.push(x);

        if self.i_stack.is_empty() {
            self.i_stack.push(0);
        }
    }

    fn pop(&mut self) -> i32 {
        let i = self.i_stack.pop().unwrap();
        let top = self.v_stack[i];

        if i == self.v_stack.len() - 1 {
            self.i_stack = Vec::new();
            self.v_stack = Vec::new();
        } else {
            self.i_stack.push(i + 1);
        }

        top
    }

    fn peek(&self) -> i32 {
        self.v_stack[*self.i_stack.last().unwrap()]
    }

    fn empty(&self) -> bool {
        self.i_stack.is_empty()
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
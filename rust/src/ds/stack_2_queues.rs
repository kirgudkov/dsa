use std::collections::VecDeque;

#[derive(Default)]
struct MyStack {
    q_1: VecDeque<i32>,
    q_2: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, x: i32) {
        // One queue should always stay empty;
        // After each peek/pop, items will be moved from one to anoter, leaving one of them empty;
        // Thus, on each push() operation we do not know which one is currntly empty.
        if self.q_1.is_empty() {
            self.q_2.push_back(x);
        } else {
            self.q_1.push_back(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.peek(true)
    }

    fn top(&mut self) -> i32 {
        self.peek(false)
    }

    fn empty(&self) -> bool {
        self.q_1.is_empty() && self.q_2.is_empty()
    }

    fn peek(&mut self, remove: bool) -> i32 {
        let mut top = 0i32;

        // Figuring out which queue is empty
        let empty = &mut self.q_2;
        let non_empty = &mut self.q_1;

        if non_empty.is_empty() {
            std::mem::swap(empty, non_empty);
        }

        // Since we're only allowed to use queues, we cannot immediately access 
        // the "back" item of the queue (actually in Rust we can, but we're emulating regular queue); 
        // So, we need to reach the end (back) of the queue and return the last item;
        // The only way to do so is to drain one queue; 

        // We're going to pour items from one queue to another;
        // Last item will be the "back" item; return it and remove, if needed;
        while !non_empty.is_empty() {
            if let Some(curr) = non_empty.pop_front() {
                top = curr;

                if remove && non_empty.is_empty() {
                    continue;
                }

                empty.push_back(curr);
            }
        }

        top
    }
}

#[cfg(test)]
mod tests {
    use crate::ds::stack_2_queues::MyStack;

    #[test]
    fn test_stack() {
        {
            let mut stack = MyStack::new();
            stack.push(1);
            stack.push(2);
            assert_eq!(stack.top(), 2);
            assert_eq!(stack.pop(), 2);
            assert!(!stack.empty());
        }

        {
            let mut stack = MyStack::new();
            stack.push(1);
            stack.pop();
            assert!(stack.empty());
        }
    }
}
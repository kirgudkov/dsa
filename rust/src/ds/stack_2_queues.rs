use std::collections::VecDeque;

struct MyStack {
    q_1: VecDeque<i32>,
    q_2: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self {
            q_1: VecDeque::new(),
            q_2: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
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

    fn peek(&mut self, pop: bool) -> i32 {
        let mut top: i32 = 0;
        let empty: &mut VecDeque<i32>;
        let non_empty: &mut VecDeque<i32>;

        if self.q_1.is_empty() {
            empty = &mut self.q_1;
            non_empty = &mut self.q_2;
        } else {
            empty = &mut self.q_2;
            non_empty = &mut self.q_1;
        }

        while !non_empty.is_empty() {
            if let Some(_top) = non_empty.pop_front() {
                top = _top;

                if pop {
                    if !non_empty.is_empty() {
                        empty.push_back(_top);
                    }
                } else {
                    empty.push_back(_top);
                }
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
        // {
        //     let mut stack = MyStack::new();
        //     stack.push(1);
        //     stack.push(2);
        //     assert_eq!(stack.top(), 2);
        //     assert_eq!(stack.pop(), 2);
        //     assert!(!stack.empty());
        // }

        {
            let mut stack = MyStack::new();
            stack.push(1);
            stack.pop();
            assert!(stack.empty());
        }
    }
}
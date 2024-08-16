struct MyCircularQueue {
    buf: Vec<i32>,
    head: Option<usize>,
    tail: Option<usize>,
    k: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            buf: vec![0; k as usize],
            head: None,
            tail: None,
            k: k as usize,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if self.is_empty() {
            self.head = Some(0);
            self.tail = Some(0);
            self.buf[0] = value;
        } else {
            self.tail = Some((self.tail.unwrap() + 1) % self.k);
            self.buf[(self.tail.unwrap()) % self.k] = value;
        }

        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        if self.head.unwrap() == self.tail.unwrap() {
            self.head = None;
            self.tail = None;
        } else {
            self.head = Some((self.head.unwrap() + 1) % self.k);
        }

        true
    }

    fn front(&self) -> i32 {
        if let Some(head) = self.head {
            self.buf[head]
        } else {
            -1
        }
    }

    fn rear(&self) -> i32 {
        if let Some(tail) = self.tail {
            self.buf[tail]
        } else {
            -1
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn is_full(&self) -> bool {
        if let (Some(head), Some(tail)) = (self.head, self.tail) {
            (tail + 1) % self.k == head
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = MyCircularQueue::new(3);
        assert!(obj.en_queue(1));
        assert!(obj.en_queue(2));
        assert!(obj.en_queue(3));
        assert!(!obj.en_queue(4));
        assert_eq!(obj.rear(), 3);
        assert!(obj.is_full());
        assert!(obj.de_queue());
        assert!(obj.en_queue(4));
        assert_eq!(obj.rear(), 4);

        assert!(obj.de_queue());
        assert!(obj.de_queue());
        assert!(obj.de_queue());
        assert!(!obj.de_queue());
        assert!(obj.is_empty());

        assert_eq!(obj.front(), -1);
        assert_eq!(obj.rear(), -1);
    }
}

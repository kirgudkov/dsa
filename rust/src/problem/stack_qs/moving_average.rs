struct MovingAverage {
    size: usize,
    avg: f64,
    q: std::collections::VecDeque<i32>,
}

impl MovingAverage {
    fn new(size: i32) -> Self {
        Self {
            avg: 0.0,
            size: size as usize,
            q: std::collections::VecDeque::new(),
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.q.len() == self.size {
            let front = self.q.pop_front().unwrap() as f64;
            self.avg = (self.avg * self.size as f64 - front + val as f64) / self.size as f64;
            self.q.push_back(val);
        } else {
            self.q.push_back(val);
            self.avg = (self.avg * (self.q.len() - 1) as f64 + val as f64) / self.q.len() as f64;
        }

        self.avg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = MovingAverage::new(3);
        assert_eq!(obj.next(1), 1.0);
        assert_eq!(obj.next(10), 5.5);
        assert_eq!(obj.next(3), 4.666666666666667);
        assert_eq!(obj.next(5), 6.0);
    }
}
struct MyHashMap {
    size: usize,
    capacity: usize,
    buckets: Vec<Vec<(i32, i32)>>,
}

impl MyHashMap {
    const DEFAULT_CAPACITY: usize = 5;

    fn new() -> Self {
        Self {
            size: 0,
            capacity: Self::DEFAULT_CAPACITY,
            buckets: vec![vec![]; Self::DEFAULT_CAPACITY],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.resize();
        let hash = self.hash(key);

        if let Some(pair) = self.buckets[hash]
            .iter_mut()
            .find(|x| x.0 == key)
        {
            pair.1 = value;
        } else {
            self.buckets[hash].push((key, value));
            self.size += 1;
        }
    }

    fn get(&self, key: i32) -> i32 {
        self.buckets[self.hash(key)]
            .iter()
            .find(|x| x.0 == key)
            .map(|x| x.1)
            .unwrap_or(-1)
    }

    fn remove(&mut self, key: i32) {
        let hash = self.hash(key);

        if let Some(pos) = self.buckets[hash]
            .iter()
            .position(|x| x.0 == key)
        {
            self.buckets[hash].remove(pos);
            self.size -= 1;
        }
    }

    fn hash(&self, key: i32) -> usize {
        (key % self.capacity as i32).unsigned_abs() as usize
    }

    fn resize(&mut self) {
        if (self.size as f32 / self.capacity as f32) < 0.75 {
            return;
        }

        self.capacity *= 2;
        let mut new_buckets: Vec<Vec<(i32, i32)>> = vec![vec![]; self.capacity];

        for bucket in &self.buckets {
            for pair in bucket {
                new_buckets[self.hash(pair.0)].push(*pair)
            }
        }

        self.buckets = new_buckets;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut hm = MyHashMap::new();
        hm.put(1, 1);
        hm.put(2, 2);
        hm.put(3, 3);
        hm.put(4, 4);
        hm.put(5, 5);
        hm.put(6, 6);
        hm.put(12, 12);
        hm.put(1, 10);
        assert_eq!(hm.get(1), 10);
        assert_eq!(hm.get(2), 2);
        hm.remove(1);
        assert_eq!(hm.get(1), -1);
    }
}
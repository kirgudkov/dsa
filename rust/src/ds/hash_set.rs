struct MyHashSet {
    size: usize,
    capacity: usize,
    buckets: Vec<Vec<i32>>,
}

impl MyHashSet {
    const DEFAULT_CAPACITY: usize = 5;

    fn new() -> Self {
        Self {
            size: 0,
            capacity: Self::DEFAULT_CAPACITY,
            buckets: vec![vec![]; Self::DEFAULT_CAPACITY],
        }
    }

    // TC is O(n) in the worst case because of duplicates check and resizing
    fn add(&mut self, key: i32) {
        if self.contains(key) {
            return;
        }

        self.resize();

        let hash = self.hash(key);
        self.buckets[hash].push(key);
        self.size += 1;
    }

    // TC is O(n) in the worst case because of checking if the key exists
    fn remove(&mut self, key: i32) {
        if !self.contains(key) {
            return;
        }

        let hash = self.hash(key);
        let index = self.buckets[hash].iter().position(|&x| x == key).unwrap();
        self.buckets[hash].remove(index);
        self.size -= 1;
    }

    // TC is O(n) in the worst case
    fn contains(&self, key: i32) -> bool {
        self.buckets[self.hash(key)].contains(&key)
    }

    fn hash(&self, key: i32) -> usize {
        key as usize % self.capacity
    }

    // TC is O(n) in the worst case due to copying all elements to the new buckets
    fn resize(&mut self) {
        if (self.size as f32 / self.capacity as f32) < 0.75 {
            return;
        }

        let new_capacity = self.capacity * 2;
        let mut new_buckets = vec![vec![]; new_capacity];

        for bucket in &self.buckets {
            for key in bucket {
                let hash = *key as usize % new_capacity;
                new_buckets[hash].push(*key);
            }
        }

        self.capacity = new_capacity;
        self.buckets = new_buckets;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_set() {
        let mut hash_set = MyHashSet::new();
        hash_set.add(1);
        hash_set.add(2);
        hash_set.add(3);
        hash_set.add(4);
        hash_set.add(5);
        hash_set.add(6);
        assert!(hash_set.contains(1));
        hash_set.remove(1);
        assert!(!hash_set.contains(1));
        hash_set.add(10);
        hash_set.add(12);
        hash_set.add(14);
        assert!(hash_set.contains(10));
        assert!(hash_set.contains(12));
        assert!(hash_set.contains(14));
    }
}
use std::collections::HashMap;
use rand::seq::{IndexedRandom};

struct RandomizedSet {
    map: HashMap<i32, usize>,
    vec: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            vec: vec![],
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }

        self.map.insert(val, self.vec.len());
        self.vec.push(val);

        true
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.map.remove(&val) {
            None => false,
            Some(removed_i) => {
                let last_i = self.vec.len() - 1;
                let last_item = self.vec[last_i];

                self.vec.swap(removed_i, last_i);
                self.vec.pop();

                if let Some(last_i) = self.map.get_mut(&last_item) {
                    *last_i = removed_i;
                }

                true
            }
        }
    }

    fn get_random(&self) -> i32 {
        *self.vec.choose(&mut rand::thread_rng()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randomized_set() {
        let mut set = RandomizedSet::new();
        assert!(!set.remove(0));
        assert!(!set.remove(0));
        assert!(set.insert(0));
        assert_eq!(set.get_random(), 0);
        assert!(set.remove(0));
        assert!(set.insert(0));
    }
}
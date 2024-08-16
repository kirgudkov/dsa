// https://leetcode.com/problems/time-based-key-value-store

#[derive(Default)]
struct TimeMap {
    hm: std::collections::HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Default::default()
    }

    // TC: O(ML) where L is the length of the key and M is the number of entries
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.hm.entry(key).or_default().push((timestamp, value));
    }

    // Time complexity: O(NM log L) where N is number of calls, M is the size of bucket and L is the length of the key
    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(entry) = self.hm.get(&key) {
            if timestamp >= entry.first().unwrap().0 {
                let mut l = 0;
                let mut r = entry.len() as i32 - 1;

                // Run until target is found or l and r overlap
                while l <= r {
                    let m = l + (r - l) / 2;

                    if entry[m as usize].0 == timestamp {
                        return entry[m as usize].1.clone();
                    }

                    if timestamp > entry[m as usize].0 {
                        l = m + 1;
                    } else {
                        r = m - 1;
                    }
                }

                return entry[r as usize].1.clone();
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_map() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(time_map.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(time_map.get("foo".to_string(), 3), "bar".to_string());
        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(time_map.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(time_map.get("foo".to_string(), 5), "bar2".to_string());

        let mut time_map = TimeMap::new();
        time_map.set("love".to_string(), "high".to_string(), 10);
        time_map.set("love".to_string(), "low".to_string(), 20);
        assert_eq!(time_map.get("love".to_string(), 5), String::new());
        assert_eq!(time_map.get("love".to_string(), 10), "high".to_string());
        assert_eq!(time_map.get("love".to_string(), 15), "high".to_string());
        assert_eq!(time_map.get("love".to_string(), 20), "low".to_string());
        assert_eq!(time_map.get("love".to_string(), 25), "low".to_string());
    }
}

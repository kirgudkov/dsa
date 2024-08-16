// https://leetcode.com/problems/snapshot-array
struct SnapshotArray {
    timestamp: i32,
    // Each item is a vector that is more like a "history" of this item;
    // Each record in the history is a tuple of timestamp and value;
    array: Vec<Vec<(/* timestamp */i32, /* value */i32)>>,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        Self {
            timestamp: 0,
            array: vec![vec![(0, 0)]; length as usize],
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        // If last history record for this item hasn't "snapped" yet, 
        // that means we can simply replace previous value to avoid duplicates;
        if self.array[index as usize].last().unwrap().0 == self.timestamp {
            self.array[index as usize].pop();
        }

        self.array[index as usize].push((self.timestamp, val))
    }

    fn snap(&mut self) -> i32 {
        self.timestamp += 1;
        self.timestamp - 1
    }

    // Binary search is available since timestamps are in increasing order;
    // Linear search would probably be fine, though bs worth mention; 
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let history = &self.array[index as usize];

        let mut l = 0;
        let mut r = history.len() - 1;

        while l <= r {
            let m = l + (r - l) / 2;

            if history[m].0 <= snap_id {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        history[r].1
    }
}

// Naive approach. 
// Gives MLE for obvious reasons but gives a good picture of a problem

// struct SnapshotArray {
//     data: Vec<i32>,
//     history: Vec<Vec<i32>>,
// }
// 
// impl SnapshotArray {
//     fn new(length: i32) -> Self {
//         Self {
//             data: vec![0; length as usize],
//             history: vec![],
//         }
//     }
// 
//     fn set(&mut self, index: i32, val: i32) {
//         self.data[index as usize] = val;
//     }
// 
//     fn snap(&mut self) -> i32 {
//         self.history.push(self.data.clone());
//         self.history.len() as i32 - 1
//     }
// 
//     fn get(&self, index: i32, snap_id: i32) -> i32 {
//         self.history[snap_id as usize][index as usize]
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_array() {
        let mut snapshot_array = SnapshotArray::new(3);
        snapshot_array.set(0, 5);
        assert_eq!(snapshot_array.snap(), 0);
        snapshot_array.set(0, 6);
        assert_eq!(snapshot_array.get(0, 0), 5);

        snapshot_array = SnapshotArray::new(1);
        snapshot_array.snap();
        snapshot_array.snap();
        assert_eq!(snapshot_array.snap(), 2);
        snapshot_array.set(0, 6);
        assert_eq!(snapshot_array.get(0, 0), 0);
        assert_eq!(snapshot_array.get(0, 3), 6);
        assert_eq!(snapshot_array.get(0, 1), 0);
        assert_eq!(snapshot_array.get(0, 50), 6);
    }
}
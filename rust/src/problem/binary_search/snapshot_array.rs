// Naive approach. Gives Memory Limit Exceeded
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

struct SnapshotArray {
    snap_id: i32,
    data: Vec<Vec<(i32, i32)>>,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        Self {
            snap_id: 0,
            data: vec![vec![(0, 0)]; length as usize],
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        if self.data[index as usize].last().unwrap().0 == self.snap_id {
            self.data[index as usize].pop();
        }

        self.data[index as usize].push((self.snap_id, val))
    }

    fn snap(&mut self) -> i32 {
        self.snap_id += 1;
        self.snap_id - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let bucket = &self.data[index as usize];

        let mut l = 0;
        let mut r = bucket.len() as i32 - 1;

        while l <= r {
            let m = l + (r - l) / 2;

            if bucket[m as usize].0 <= snap_id {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        bucket[l as usize - 1].1
    }
}

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

        let mut snapshot_array = SnapshotArray::new(1);
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
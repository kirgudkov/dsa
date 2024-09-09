pub trait Neighbors<T> {
    fn neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)>;
}

impl<T> Neighbors<T> for Vec<Vec<T>> {
    fn neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().filter_map(|&(di, dj)| {
            let ni = i as isize + di;
            let nj = j as isize + dj;

            (ni >= 0 && ni < self.len() as isize && nj >= 0 && nj < self[0].len() as isize)
                .then_some((ni as usize, nj as usize))
        }).collect()
    }
}

pub fn partition_in_place<T>(slice: &mut [T], f: fn(&T) -> bool) -> usize
where
    T: Sized + Copy,
{
    let mut partition_i = 0;

    for i in 0..slice.len() {
        if f(&slice[i]) {
            slice.swap(partition_i, i);
            partition_i += 1;
        }
    }

    partition_i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let m = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(m.neighbors(0, 0), vec![(1, 0), (0, 1)]);
        assert_eq!(m.neighbors(0, 1), vec![(1, 1), (0, 0)]);
        assert_eq!(m.neighbors(1, 0), vec![(0, 0), (1, 1)]);
        assert_eq!(m.neighbors(1, 1), vec![(0, 1), (1, 0)]);
        assert_eq!(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]].neighbors(1, 1), vec![(2, 1), (0, 1), (1, 2), (1, 0)]);
    }

    #[test]
    fn test_partition_in_place() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let partition_i = partition_in_place(&mut nums, |&i| i != 0);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
        assert_eq!(partition_i, 3);

        let mut nums = vec![2, 1, 0, 3, 12];
        let partition_i = partition_in_place(&mut nums, |&i| i != 0);
        assert_eq!(nums, vec![2, 1, 3, 12, 0]);
        assert_eq!(partition_i, 4);

        let mut nums = vec![0];
        let partition_i = partition_in_place(&mut nums, |&i| i != 0);
        assert_eq!(nums, vec![0]);
        assert_eq!(partition_i, 0);

        let mut nums = vec![1];
        let partition_i = partition_in_place(&mut nums, |&i| i != 0);
        assert_eq!(nums, vec![1]);
        assert_eq!(partition_i, 1);

        let mut nums = vec![1, 0, 1];
        let partition_i = partition_in_place(&mut nums, |&i| i != 0);
        assert_eq!(nums, vec![1, 1, 0]);
        assert_eq!(partition_i, 2);
    }
}

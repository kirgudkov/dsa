pub fn neighbors(i: usize, j: usize, grid: &[Vec<i32>]) -> Vec<(usize, usize)> {
    let mut result = vec![];

    [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().for_each(|&(di, dj)| {
        let i = i as isize + di;
        let j = j as isize + dj;

        if i >= 0 && i < grid.len() as isize && j >= 0 && j < grid[0].len() as isize {
            result.push((i as usize, j as usize));
        }
    });

    result
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
        assert_eq!(neighbors(0, 0, &[vec![1, 2], vec![3, 4]]), vec![(1, 0), (0, 1)]);
        assert_eq!(neighbors(0, 1, &[vec![1, 2], vec![3, 4]]), vec![(1, 1), (0, 0)]);
        assert_eq!(neighbors(1, 0, &[vec![1, 2], vec![3, 4]]), vec![(0, 0), (1, 1)]);
        assert_eq!(neighbors(1, 1, &[vec![1, 2], vec![3, 4]]), vec![(0, 1), (1, 0)]);
        assert_eq!(neighbors(1, 1, &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![(2, 1), (0, 1), (1, 2), (1, 0)]);
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

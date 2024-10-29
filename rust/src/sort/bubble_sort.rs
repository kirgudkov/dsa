fn bubble_sort<T>(slice: &mut [T])
where
    T: PartialOrd + Clone,
{
    for i in 0..slice.len() {
        for j in 0..slice.len() - i - 1 {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums = vec![5, 2, 3, 1, 4];
        bubble_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }
}

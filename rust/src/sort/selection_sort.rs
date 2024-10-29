fn selection_sort<T>(slice: &mut [T])
where
    T: Ord + Clone,
{
    for i in 0..slice.len() - 1 {
        if let Some((j, _)) = slice.iter().enumerate().skip(i).min_by_key(|&(_, val)| val) {
            slice.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut arr = [4, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4]);
    }
}

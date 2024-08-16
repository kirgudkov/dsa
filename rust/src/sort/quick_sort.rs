mod hoare {
    pub fn quicksort<T>(slice: &mut [T])
    where
        T: PartialOrd + Clone,
    {
        sort(slice, 0, slice.len() - 1);
    }

    fn sort<T>(slice: &mut [T], l: usize, r: usize)
    where
        T: PartialOrd + Clone,
    {
        if l < r {
            let p = partition(slice, l, r);
            sort(slice, l, p);
            sort(slice, p + 1, r);
        }
    }

    // The Hoare partition scheme (second implementation) is generally more efficient in practice,
    // as it does three times fewer swaps on average;
    // This implementation is slightly more robust against the worst-case scenario (already sorted array) due to its pivot selection.
    fn partition<T>(slice: &mut [T], mut l: usize, mut r: usize) -> usize
    where
        T: PartialOrd + Clone,
    {
        // middle-of-three pivot selection to mitigate worst-case scenarios (already sorted array)
        let pivot = slice[l + (r - l) / 2].clone();

        loop {
            while slice[l] < pivot { l += 1; }
            while slice[r] > pivot { r -= 1; }

            if l >= r {
                return r;
            }

            slice.swap(l, r);
        }
    }
}

mod lomuto {
    pub fn quicksort<T: PartialOrd>(slice: &mut [T]) {
        if slice.len() < 2 {
            return;
        }

        let (l, r) = partition(slice);
        quicksort(l);
        quicksort(&mut r[1..]); // strip the pivot
    }

    fn partition<T: PartialOrd>(slice: &mut [T]) -> (&mut [T], &mut [T]) {
        let pivot = slice.len() - 1;
        let mut i = 0;

        for j in 0..pivot {
            if slice[j] <= slice[pivot] {
                slice.swap(i, j);
                i += 1;
            }
        }

        // After the loop, slice is somewhat: [p-, p-, i, p+, p+, p];
        // The index i keeps track of where the "less than or equal to" section ends;
        // So we additionaly swap pivot with the ith position:
        slice.swap(i, pivot);
        // After this swap, the slice is properly partitioned:
        // slice[0..i] contains all elements <= pivot
        // slice[i] is the pivot in its final sorted position
        // slice[i+1..] contains all elements > pivot

        slice.split_at_mut(i)
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::quick_sort::hoare::quicksort as hoare_quicksort;
    use crate::sort::quick_sort::lomuto::quicksort as lomuto_quicksort;

    #[test]
    fn test_lomuto() {
        let mut input = [4, 3, 2, 1];
        lomuto_quicksort(&mut input);
        assert_eq!(input, [1, 2, 3, 4]);

        let mut input = [4, 3, 2, 1, 5];
        lomuto_quicksort(&mut input);
        assert_eq!(input, [1, 2, 3, 4, 5]);

        let mut input = [1];
        lomuto_quicksort(&mut input);
        assert_eq!(input, [1]);
    }

    #[test]
    fn test_hoare() {
        let mut input = [4, 3, 2, 1];
        hoare_quicksort(&mut input);
        assert_eq!(input, [1, 2, 3, 4]);

        let mut input = [4, 3, 2, 1, 5];
        hoare_quicksort(&mut input);
        assert_eq!(input, [1, 2, 3, 4, 5]);

        let mut input = [1];
        hoare_quicksort(&mut input);
        assert_eq!(input, [1]);

        let mut input = [4, 2, 7, 1, 0];
        hoare_quicksort(&mut input);
        assert_eq!(input, [0, 1, 2, 4, 7]);
    }
}

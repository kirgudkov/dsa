pub fn quick_sort(input: &mut [i32]) {
    sort(input, 0, input.len() - 1);
}

fn sort(input: &mut [i32], l: usize, r: usize) {
    if l < r {
        let p = partition(input, l, r);
        sort(input, l, p);
        sort(input, p + 1, r);
    }
}

fn partition(input: &mut [i32], mut l: usize, mut r: usize) -> usize {
    let pivot = input[l];

    loop {
        while input[l] < pivot {
            l += 1;
        }

        while input[r] > pivot {
            r -= 1;
        }

        if l >= r {
            return r;
        }

        input.swap(l, r);
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::quick_sort::quick_sort;

    #[test]
    fn test_quick_sort() {
        let mut input = [4, 3, 2, 1];
        quick_sort(&mut input);
        assert_eq!(input, [1, 2, 3, 4]);

        let mut input = [4, 3, 2, 1, 5];
        quick_sort(&mut input);
        assert_eq!(input, [1, 2, 3, 4, 5]);

        let mut input = [1];
        quick_sort(&mut input);
        assert_eq!(input, [1]);
    }
}

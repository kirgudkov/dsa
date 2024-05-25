pub fn quick_sort(input: &mut [i32]) {
    sort(input, 0, input.len() - 1);
}

fn sort(input: &mut [i32], l: usize, h: usize) {
    if l < h {
        let p = partition(input, l, h);
        sort(input, l, p);
        sort(input, p + 1, h);
    }
}

fn partition(input: &mut [i32], l: usize, h: usize) -> usize {
    let pivot = input[l];
    let mut i = l;
    let mut j = h;
    
    loop {
        while input[i] < pivot {
            i += 1;
        }

        while input[j] > pivot {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        input.swap(i, j);
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

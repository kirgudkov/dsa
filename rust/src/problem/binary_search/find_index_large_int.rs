struct ArrayReader {
    arr: Vec<i32>,
}

impl ArrayReader {
    #[allow(non_snake_case)]
    pub fn compareSub(&self, a: i32, b: i32, c: i32, d: i32) -> std::cmp::Ordering {
        let l_sum: i32 = self.arr[(a as usize)..=(b as usize)].iter().sum();
        let r_sum: i32 = self.arr[(c as usize)..=(d as usize)].iter().sum();

        l_sum.cmp(&r_sum)
    }

    pub fn length(&self) -> i32 {
        self.arr.len() as i32
    }
}

fn get_index(reader: &ArrayReader) -> i32 {
    let mut l = 0;
    let mut r = reader.length() - 1;

    while l <= r {
        let m = l + (r - l) / 2;
        let offset = if (r - l) % 2 == 0 { 0 } else { 1 };

        match reader.compareSub(l, m, m + offset, r) {
            std::cmp::Ordering::Greater => r = m,
            std::cmp::Ordering::Less => l = m + 1,
            std::cmp::Ordering::Equal => return m
        }
    }

    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_reader() {
        let reader = ArrayReader { arr: vec![7, 7, 7, 7, 10, 7, 7, 7] };
        assert_eq!(reader.compareSub(0, 3, 4, 7), std::cmp::Ordering::Less);
        assert_eq!(reader.compareSub(0, 2, 3, 7), std::cmp::Ordering::Less);
        assert_eq!(reader.compareSub(0, 4, 5, 7), std::cmp::Ordering::Greater);
        assert_eq!(reader.compareSub(0, 1, 2, 3), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_get_index() {
        let reader = ArrayReader { arr: vec![7, 7, 7, 7, 10, 7, 7, 7] };
        assert_eq!(get_index(&reader), 4);

        let reader = ArrayReader { arr: vec![10, 7, 7, 7, 7, 7, 7, 7] };
        assert_eq!(get_index(&reader), 0);

        let reader = ArrayReader { arr: vec![7, 7, 7, 7, 7, 7, 7, 10] };
        assert_eq!(get_index(&reader), 7);

        let reader = ArrayReader { arr: vec![6, 6, 12] };
        assert_eq!(get_index(&reader), 2);

        let reader = ArrayReader { arr: vec![1, 2, 1, 1, 1, 1, 1, 1, 1] };
        assert_eq!(get_index(&reader), 1);
    }
}
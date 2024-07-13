struct ArrayReader {
    arr: Vec<i32>,
}

impl ArrayReader {
    #[allow(non_snake_case)]
    pub fn compareSub(&self, l: i32, r: i32, x: i32, y: i32) -> i32 {
        match &self.arr[(l as usize)..=(r as usize)].iter().sum::<i32>().cmp(&self.arr[(x as usize)..=(y as usize)].iter().sum::<i32>()) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        }
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
            1 => {
                r = m;
            }
            -1 => {
                l = m + 1;
            }
            _ => {
                return m;
            }
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
        assert_eq!(reader.compareSub(0, 3, 4, 7), -1);
        assert_eq!(reader.compareSub(0, 2, 3, 7), -1);
        assert_eq!(reader.compareSub(0, 4, 5, 7), 1);
        assert_eq!(reader.compareSub(0, 1, 2, 3), 0);
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
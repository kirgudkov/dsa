use std::collections::BinaryHeap;

pub fn reorganize_string(s: String) -> String {
    let mut heap = BinaryHeap::<(i32, char)>::new();

    s.as_bytes().iter()
        .fold(vec![0; 26], |mut acc, b| {
            acc[(b - b'a') as usize] += 1;
            acc
        })
        .iter().enumerate()
        .for_each(|(i, &count)| {
            if count > 0 {
                heap.push((count, (b'a' + i as u8) as char));
            }
        });

    let mut res = String::new();

    while heap.len() > 1 {
        let (f1, c1) = heap.pop().unwrap();
        let (f2, c2) = heap.pop().unwrap();

        res.push_str(&format!("{}{}", c1, c2));

        if f1 > 1 { heap.push((f1 - 1, c1)) }
        if f2 > 1 { heap.push((f2 - 1, c2)) }
    }

    if let Some((f, c)) = heap.pop() {
        if f > 1 {
            return String::from("");
        } else {
            res.push(c);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(super::reorganize_string("aab".to_string()), "aba".to_string());
        assert_eq!(super::reorganize_string("aaab".to_string()), "".to_string());
    }
}
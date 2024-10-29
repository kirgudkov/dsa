use std::collections::{BinaryHeap, HashMap};

pub fn reorganize_string(s: String) -> String {
    let mut result = String::new();

    let mut queue: BinaryHeap<(i32, char)> = s.chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .map(|(c, f)| (f, c))
        .collect();

    while queue.len() > 1 {
        let (f1, c1) = queue.pop().unwrap();
        let (f2, c2) = queue.pop().unwrap();

        result.push_str(&format!("{}{}", c1, c2));

        if f1 > 1 { queue.push((f1 - 1, c1)); }
        if f2 > 1 { queue.push((f2 - 1, c2)); }
    }

    match queue.pop() {
        None => result,
        Some((1, c)) => format!("{}{}", result, c),
        Some(_) => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(super::reorganize_string("aab".to_string()), "aba".to_string());
        assert_eq!(super::reorganize_string("aaab".to_string()), "".to_string());
    }
}
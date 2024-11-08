use std::collections::{BinaryHeap, HashMap};

pub fn rearrange_string(s: String, k: i32) -> String {
    if k == 0 {
        return s;
    }

    let mut result = String::new();

    let mut queue = s.chars()
        .fold(HashMap::<char, usize>::new(), |mut freqs, char| {
            *freqs.entry(char).or_default() += 1;
            freqs
        })
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect::<BinaryHeap<(usize, char)>>();

    while !queue.is_empty() {
        let mut pushed = 0;
        let mut stash = vec![];

        while pushed < k {
            if let Some((freq, char)) = queue.pop() {
                result.push(char);

                if freq - 1 > 0 {
                    stash.push((freq - 1, char));
                }
            } else if !stash.is_empty() && pushed < k {
                return "".to_string();
            }

            pushed += 1;
        }

        stash.iter()
            .for_each(|&pair| queue.push(pair));
    }

    result
}

use crate::ds::disjoint_set::DisjointSet;

// https://leetcode.com/problems/smallest-string-with-swaps/
pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
    let mut res: Vec<char> = s.chars().collect();

    let mut ds = DisjointSet::new(res.len());
    pairs.iter().for_each(|pair| {
        ds.union(pair[0] as usize, pair[1] as usize);
    });

    let mut sets = vec![vec![]; res.len()];
    (0..res.len()).for_each(|i| {
        sets[ds.find(i)].push(i);
    });

    sets.iter().for_each(|set| {
        let mut chars: Vec<char> = set.iter()
            .map(|&id| res[id])
            .collect();

        chars.sort_unstable();

        set.iter().zip(chars.iter())
            .for_each(|(&pos, &ch)| res[pos] = ch);
    });

    res.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_string_with_swaps() {
        let s = "dcab".to_string();
        let pairs = vec![vec![0, 3], vec![1, 2]];
        assert_eq!(smallest_string_with_swaps(s, pairs), "bacd".to_string());

        let s = "dcab".to_string();
        let pairs = vec![vec![0, 3], vec![1, 2], vec![0, 2]];
        assert_eq!(smallest_string_with_swaps(s, pairs), "abcd".to_string());
    }
}
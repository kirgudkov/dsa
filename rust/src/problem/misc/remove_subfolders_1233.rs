pub fn remove_subfolders(mut folders: Vec<String>) -> Vec<String> {
    // O(N Log N), but since this is a lexicographical sorting, string comparison could take up to the longest string length L
    // This give us a O(N * L * Log N) time complexity
    folders.sort_unstable();
    let mut result = vec![folders[0].clone()];

    // This loop runs N times and performs string comparison of max length L
    // So the time complexity is O(N * L)
    for i in 1..folders.len() {
        if folders[i].starts_with(&(result.last().unwrap().clone() + "/")) {
            continue;
        }

        result.push(folders[i].clone());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let folders = vec![
            String::from("/a"),
            String::from("/a/b"),
            String::from("/a/b/c"),
            String::from("/a/b/d"),
            String::from("/b"),
            String::from("/b/c"),
            String::from("/b/d"),
            String::from("/b/d/e"),
        ];

        let result = remove_subfolders(folders);

        assert_eq!(result, vec![
            String::from("/a"),
            String::from("/b"),
        ]);
    }
}

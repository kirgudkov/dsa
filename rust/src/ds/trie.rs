#[derive(Default, PartialEq, Debug)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    terminal: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn from(words: Vec<String>) -> Self {
        let mut trie = Self::new();

        for word in words {
            let mut node = &mut trie;

            for c in word.chars() {
                node = node.children[c as usize - 'a' as usize]
                    .get_or_insert_with(|| Box::new(Trie::new()));
            }

            node.terminal = true;
        }

        trie
    }

    fn has_prefix(&self, prefix: &str) -> bool {
        self.search_prefix(prefix).is_some()
    }

    fn search_prefix(&self, prefix: &str) -> Option<&Trie> {
        let mut node = self;

        for c in prefix.chars() {
            let idx = c as usize - 'a' as usize;
            if let Some(child) = &node.children[idx] {
                node = child;
            } else {
                return None;
            }
        }

        Some(node)
    }

    fn has_word(&self, word: &str) -> bool {
        if let Some(node) = self.search_prefix(word) {
            return node.terminal;
        }

        false
    }

    fn search_word(&self, word: &str) -> Option<&Trie> {
        self.search_prefix(word).filter(|node| node.terminal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let trie = Trie::from(vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()]);
        assert!(trie.has_prefix("a"));
        assert!(trie.has_prefix("ap"));
        assert!(trie.has_prefix("app"));
        assert!(trie.has_prefix("appl"));
        assert!(trie.has_prefix("apple"));
        assert!(!trie.has_prefix("apples"));
        assert!(trie.has_prefix("b"));
        assert!(trie.has_prefix("c"));
        assert!(!trie.has_prefix("d"));
        assert!(trie.has_word("apple"));
        assert!(trie.has_word("banana"));
        assert!(trie.has_word("cherry"));
        assert!(!trie.has_word("app"));
        assert!(!trie.has_word("ban"));
        assert!(!trie.has_word("cher"));
        assert!(!trie.has_word("apples"));
        assert!(!trie.has_word("bananas"));
        assert!(!trie.has_word("cherries"));
        assert!(trie.search_word("apple").is_some());
        assert!(trie.search_word("banana").is_some());
        assert!(trie.search_word("cherry").is_some());
        assert_eq!(trie.search_word("app"), None);
        assert_eq!(trie.search_word("ban"), None);
        assert_eq!(trie.search_word("cher"), None);
        assert_eq!(trie.search_word("apples"), None);
        assert_eq!(trie.search_word("bananas"), None);
        assert_eq!(trie.search_word("cherries"), None);
    }
}

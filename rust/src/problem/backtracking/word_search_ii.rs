// pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
//     Ctx::new(board).find_words(words)
// }
// 
// struct Ctx {
//     buf: String,
//     board: Vec<Vec<char>>,
//     visited: HashSet<(usize, usize)>,
// }
// 
// // Fully working pure backtracking implementation but not quite efficient and not getting accepted by Leetcode due to TLE.
// // More efficient approach is to use Trie which is implemented below, but it's still not efficient enough getting TLE sometimes.
// impl Ctx {
//     fn new(board: Vec<Vec<char>>) -> Self {
//         Self {
//             board,
//             buf: String::new(),
//             visited: HashSet::new(),
//         }
//     }
// 
//     fn find_words(&mut self, words: Vec<String>) -> Vec<String> {
//         let mut result = vec![];
// 
//         for word in words {
//             let chars = word.chars().collect::<Vec<char>>();
// 
//             'board: for i in 0..self.board.len() {
//                 for j in 0..self.board[i].len() {
//                     if self.board[i][j] == chars[0] {
//                         self.buf.clear();
//                         self.visited.clear();
// 
//                         if self.backtrack(&chars, i, j, 0) {
//                             result.push(self.buf.clone());
//                             break 'board;
//                         }
//                     }
//                 }
//             }
//         }
// 
//         result
//     }
// 
//     fn backtrack(&mut self, chars: &[char], i: usize, j: usize, k: usize) -> bool {
//         if k == chars.len() {
//             return true;
//         }
// 
//         if i == self.board.len()
//             || j == self.board[0].len()
//             || self.visited.contains(&(i, j))
//             || self.board[i][j] != chars[k]
//         {
//             return false;
//         }
// 
//         self.visited.insert((i, j));
//         self.buf.push(chars[k]);
// 
//         if self.backtrack(chars, i + 1, j, k + 1)
//             || self.backtrack(chars, i.saturating_sub(1), j, k + 1)
//             || self.backtrack(chars, i, j + 1, k + 1)
//             || self.backtrack(chars, i, j.saturating_sub(1), k + 1)
//         {
//             return true;
//         }
// 
//         self.visited.remove(&(i, j));
//         self.buf.pop();
// 
//         false
//     }
// }

pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    Ctx::new(board, words).find_words()
}

struct Ctx {
    buf: String,
    result: Vec<String>,
    board: Vec<Vec<char>>,
    trie: Trie,
}

impl Ctx {
    fn new(board: Vec<Vec<char>>, words: Vec<String>) -> Self {
        Self {
            board,
            buf: String::new(),
            result: vec![],
            trie: Trie::from(words),
        }
    }

    fn find_words(&mut self) -> Vec<String> {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                self.backtrack("", i, j);
            }
        }

        self.result.clone()
    }

    fn backtrack(&mut self, prefix: &str, i: usize, j: usize) {
        if !self.in_bounds(i, j) || self.board[i][j] == '#' {
            return;
        }

        let ch = self.board[i][j];
        self.board[i][j] = '#';

        let prefix = format!("{}{}", prefix, ch);

        if let Some(ref mut node) = self.trie.search_prefix(&prefix) {
            if node.terminal {
                node.terminal = false;
                self.result.push(prefix.clone());
            }

            self.backtrack(&prefix, i + 1, j);
            self.backtrack(&prefix, i.saturating_sub(1), j);
            self.backtrack(&prefix, i, j + 1);
            self.backtrack(&prefix, i, j.saturating_sub(1));
        }

        self.board[i][j] = ch;
    }

    fn in_bounds(&self, i: usize, j: usize) -> bool {
        i < self.board.len() && j < self.board[0].len()
    }
}

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    terminal: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn from(words: Vec<String>) -> Self {
        let mut node = Trie::new();

        for word in words {
            let mut node = &mut node;

            for c in word.chars() {
                node = node.children[c as usize - 'a' as usize]
                    .get_or_insert_with(|| Box::new(Trie::new()));
            }

            node.terminal = true;
        }

        node
    }

    fn search_prefix(&mut self, prefix: &str) -> Option<&mut Trie> {
        let mut node = self;

        for c in prefix.chars() {
            node = match node.children[c as usize - 'a' as usize].as_deref_mut() {
                Some(child) => child,
                None => return None,
            };
        }

        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_words() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let mut result = find_words(board, vec!["oath".to_string(), "pea".to_string(), "eat".to_string(), "rain".to_string(), "hklf".to_string(), "hf".to_string()]);
        result.sort();
        let mut expected = vec!["oath".to_string(), "eat".to_string(), "hklf".to_string(), "hf".to_string()];
        expected.sort();
        assert_eq!(result, expected);

        let board = vec![
            vec!['o', 'a', 'n', 'n'],
            vec!['o', 't', 'a', 'e'],
            vec!['a', 'h', 'k', 'r'],
            vec!['a', 'f', 'l', 'v'],
        ];
        let words = vec!["oa".to_string(), "oaa".to_string()];
        let result = find_words(board, words);
        assert_eq!(result, vec!["oa".to_string(), "oaa".to_string()]);

        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words = vec!["oath".to_string(), "pea".to_string(), "eat".to_string(), "rain".to_string()];
        let result = find_words(board, words);
        assert_eq!(result, vec!["oath".to_string(), "eat".to_string()]);

        let board = vec![
            vec!['a', 'b'],
            vec!['c', 'd'],
        ];
        let words = vec!["abcd".to_string()];
        let result = find_words(board, words);
        assert_eq!(result, Vec::<String>::new());
    }
}
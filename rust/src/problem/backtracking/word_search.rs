// https://leetcode.com/problems/word-search
// TC is O(m * n * 4^s) where m is the number of rows, n is the number of columns, and s is the length of the word.
pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
    let chars = word.chars().collect::<Vec<char>>();

    fn backtrack(board: &mut [Vec<char>], chars: &[char], i: usize, j: usize, k: usize) -> bool {
        if k == chars.len() {
            return true;
        }

        if i == board.len() || j == board[0].len() || board[i][j] != chars[k] {
            return false;
        }

        let tmp = board[i][j];
        board[i][j] = '#';

        if backtrack(board, chars, i + 1, j, k + 1)
            || backtrack(board, chars, i.saturating_sub(1), j, k + 1)
            || backtrack(board, chars, i, j + 1, k + 1)
            || backtrack(board, chars, i, j.saturating_sub(1), k + 1)
        {
            return true;
        }

        board[i][j] = tmp;
        false
    }

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == chars[0] && backtrack(&mut board, &chars, i, j, 0) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_search() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(exist(board, "ABCCED".to_string()));

        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(exist(board, "SEE".to_string()));

        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(!exist(board, "ABCB".to_string()));
    }
}

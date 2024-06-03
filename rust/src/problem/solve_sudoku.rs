// TC is O(9^m) where m is the number of empty cells in the board;
// SC is O(n^2) where n is the number of cells in the board + O(m) for the recursion stack, where m is the number of empty cells in the board.
pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    backtrack(0, 0, board);
}

fn backtrack(i: usize, j: usize, board: &mut Vec<Vec<char>>) -> bool {
    if i == board.len() {
        return true;
    }

    let (next_i, next_j) = if j == board[i].len() - 1 {
        (i + 1, 0)
    } else {
        (i, j + 1)
    };

    if board[i][j] != '.' {
        return backtrack(next_i, next_j, board);
    }

    for num in '1'..='9' {
        if !is_valid(i, j, num, board) {
            continue;
        }

        board[i][j] = num;

        if backtrack(i, j, board) {
            return true;
        } else {
            board[i][j] = '.';
        }
    }

    false
}

fn is_valid(i: usize, j: usize, num: char, board: &[Vec<char>]) -> bool {
    for k in 0..9 {
        if board[i][k] == num || board[k][j] == num {
            return false;
        }
    }

    let start_row = (i / 3) * 3;
    let start_col = (j / 3) * 3;

    for row in board.iter().skip(start_row).take(3) {
        for cell in row.iter().skip(start_col).take(3) {
            if *cell == num {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::problem::solve_sudoku::solve_sudoku;

    #[test]
    fn test() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let res = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        solve_sudoku(&mut board);
        assert_eq!(board, res);
    }
}
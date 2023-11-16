// Problem

// Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:

// Each row must contain the digits 1-9 without repetition.
// Each column must contain the digits 1-9 without repetition.
// Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.

//Notes:

// Complexity:
// Time: O(9^2)
// Space: O(9^2)
use std::collections::HashSet;

#[allow(dead_code)]
fn valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut cols = HashSet::new();
    let mut rows = HashSet::new();
    let mut squares = HashSet::new();
    for row in 0..9 {
        for col in 0..9 {
            let chr = board[row][col];
            if chr.is_numeric() {
                if !cols.insert((col, chr)) {
                    return false;
                }
                if !rows.insert((row, chr)) {
                    return false;
                }
                if !squares.insert((row / 3, col / 3, chr)) {
                    return false;
                }
            }
        }
    }
    true
}


#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_case_1() {
        let board = vec![
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

        assert!(valid_sudoku(board));
    }
}

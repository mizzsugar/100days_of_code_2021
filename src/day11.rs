// https://leetcode.com/problems/valid-sudoku
// https://leetcode.com/problems/valid-sudoku/discuss/1076764/Rust-or-Easy-to-understand

use std::collections::{HashMap, HashSet};

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    valid_rows(&board) && valid_columns(&board) && valid_boxes(&board)
}

fn valid_rows(board: &Vec<Vec<char>>) -> bool {
    let mut set = HashSet::with_capacity(9);
    for row in board {
        set.clear();
        for elem in row {
            if *elem != '.' {
                if set.contains(&elem) {
                    return false;
                }
                set.insert(elem);
            }
        }
    }
    return true;
}

fn valid_columns(board: &Vec<Vec<char>>) -> bool {
    let mut map: HashMap<usize, HashSet<char>> = HashMap::with_capacity(9);
    for row in board {
        for (j, elem) in row.iter().enumerate() {
            if *elem != '.' {
                let set = map.entry(j).or_insert(HashSet::with_capacity(9));
                if set.contains(elem) {
                    return false;
                }
                set.insert(*elem);
            }
        }
    }
    true
}

fn valid_boxes(board: &Vec<Vec<char>>) -> bool {
    let mut map = HashMap::with_capacity(9);
    for (i, row) in board.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if *elem != '.' {
                let box_index = (i / 3) * 3 + j / 3;
                let set = map.entry(box_index).or_insert(HashSet::with_capacity(9));
                if set.contains(elem) {
                    return false;
                }
                set.insert(elem);
            }
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::is_valid_sudoku;
    #[test]
    fn test_is_valid_sudoku() {
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
        assert!(is_valid_sudoku(board));
    }

    #[test]
    fn test_not_valid() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!is_valid_sudoku(board));
    }
}

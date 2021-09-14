fn main() {
    println!(
        "{}",
        Solution::is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ])
    );
    println!(
        "{}",
        Solution::is_valid_sudoku(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ])
    );
    println!(
        "{}",
        Solution::is_valid_sudoku(vec![
            vec!['.', '8', '7', '6', '5', '4', '3', '2', '1'],
            vec!['2', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['3', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['4', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['6', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['7', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['8', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['9', '.', '.', '.', '.', '.', '.', '.', '.'],
        ])
    );
}

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for x in 0..9 {
            for y in 0..9 {
                let value = board[x][y];
                if value == '.' {
                    continue;
                }
                let mut alt_values = 0u16;
                for j in 0..9 {
                    if y == j {
                        continue;
                    }
                    let v = board[x][j];
                    if v == '.' {
                        continue;
                    }
                    alt_values = alt_values | MASKS[(v as u8 - b'0') as usize - 1];
                    if MASKS[(value as u8 - b'0' - 1) as usize] & alt_values > 0 {
                        return false;
                    }
                }
                for i in 0..9 {
                    if x == i {
                        continue;
                    }
                    let v = board[i][y];
                    if v == '.' {
                        continue;
                    }
                    alt_values = alt_values | MASKS[(v as u8 - b'0') as usize - 1];
                    if MASKS[(value as u8 - b'0' - 1) as usize] & alt_values > 0 {
                        return false;
                    }
                }
                for i in (x / 3 * 3)..(x / 3 * 3 + 3) {
                    for j in (y / 3 * 3)..(y / 3 * 3 + 3) {
                        if x == i && y == j {
                            continue;
                        }
                        let v = board[i][j];
                        if v == '.' {
                            continue;
                        }
                        alt_values = alt_values | MASKS[(v as u8 - b'0') as usize - 1];
                        if MASKS[(value as u8 - b'0' - 1) as usize] & alt_values > 0 {
                            return false;
                        }
                    }
                }
            }
        }
        return true;
    }
}

const MASKS: [u16; 9] = [1, 2, 4, 8, 16, 32, 64, 128, 256];

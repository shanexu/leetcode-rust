fn main() {}

struct Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let rows = board.len();
        let cols = board[0].len();
        let mut word = word.as_bytes().to_vec();

        let mut board_freq = vec![0; 128];
        for i in 0..rows {
            for j in 0..cols {
                board_freq[board[i][j] as usize] += 1;
            }
        }

        for &b in word.iter() {
            if board_freq[b as usize] == 0 {
                return false;
            }
            board_freq[b as usize] -= 1;
        }

        if board_freq[word[0] as usize] > board_freq[word[word.len() - 1] as usize] {
            word.reverse();
        };

        for i in 0..rows {
            for j in 0..cols {
                if dfs(i, j, 0, &mut board, &word) {
                    return true;
                }
            }
        }
        false
    }
}

#[inline(always)]
fn dfs(row: usize, col: usize, index: usize, board: &mut Vec<Vec<char>>, word: &[u8]) -> bool {
    let rows = board.len();
    let cols = board[0].len();
    if index == word.len() - 1 {
        return board[row][col] == word[index] as char;
    }
    if board[row][col] != word[index] as char {
        return false;
    }
    let temp_char = board[row][col];
    board[row][col] = '\0';
    let directions = vec![-1, 0, 1, 0, -1];
    for d in 0..4 {
        let new_row = row as i32 + directions[d];
        let new_col = col as i32 + directions[d + 1];
        if new_row >= 0
            && new_row < rows as i32
            && new_col >= 0
            && new_col < cols as i32
            && board[new_row as usize][new_col as usize] != '\0'
        {
            if dfs(new_row as usize, new_col as usize, index + 1, board, word) {
                return true;
            }
        }
    }
    board[row][col] = temp_char;
    false
}

fn main() {
    let mut board = vec![
        vec!['X', 'O', 'X', 'O', 'X', 'O'],
        vec!['O', 'X', 'O', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'O', 'X', 'O'],
        vec!['O', 'X', 'O', 'X', 'O', 'X'],
    ];
    Solution::solve(&mut board);
    println!("{:?}", board);
}

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let rows = board.len();
        let cols = board[0].len();
        let mut queue = VecDeque::new();
        for i in 0..rows {
            for j in 0..cols {
                if i == 0 || j == 0 || i == rows - 1 || j == cols - 1 {
                    if board[i][j] == 'O' {
                        queue.push_back((i, j));
                        while let Some((x, y)) = queue.pop_front() {
                            if board[x][y] == 'O' {
                                board[x][y] = 'Q';
                                let mut dx = 0;
                                let mut dy = 1;
                                for _ in 0..4 {
                                    let new_x = x as i32 + dx;
                                    let new_y = y as i32 + dy;
                                    (dx, dy) = (dy, -dx);
                                    if new_x >= 0
                                        && new_x < rows as i32
                                        && new_y >= 0
                                        && new_y < cols as i32
                                    {
                                        let new_x = new_x as usize;
                                        let new_y = new_y as usize;
                                        if board[new_x][new_y] == 'O' {
                                            queue.push_back((new_x, new_y));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        for i in 0..rows {
            for j in 0..cols {
                let k = board[i][j];
                if k == 'O' {
                    board[i][j] = 'X';
                } else if k == 'Q' {
                    board[i][j] = 'O';
                }
            }
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ])
    );
}

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut queue = VecDeque::new();
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '1' {
                    ans += 1;
                    queue.push_back((i, j));
                    while let Some((row, col)) = queue.pop_front() {
                        if grid[row][col] == '1' {
                            grid[row][col] = '0';
                            let mut dx = 0;
                            let mut dy = 1;
                            for _ in 0..4 {
                                let new_row = row as i32 + dx;
                                let new_col = col as i32 + dy;
                                (dx, dy) = (dy, -dx);
                                if new_row >= 0
                                    && new_col >= 0
                                    && new_row < rows as i32
                                    && new_col < cols as i32
                                {
                                    let new_row = new_row as usize;
                                    let new_col = new_col as usize;
                                    if grid[new_row][new_col] == '1' {
                                        queue.push_back((new_row, new_col));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        ans
    }
}

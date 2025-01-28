fn main() {
    println!(
        "{}",
        Solution::max_area_of_island(vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ])
    )
}

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut queue = VecDeque::new();
        let rows = grid.len();
        let cols = grid[0].len();
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 1 {
                    let mut s = 0;
                    queue.push_back((i, j));
                    while let Some((row, col)) = queue.pop_front() {
                        if grid[row][col] == 0 {
                            continue;
                        }
                        s += 1;
                        grid[row][col] = 0;
                        for k in 0..4 {
                            let new_row = row as i32 + DIRECTIONS[k];
                            let new_col = col as i32 + DIRECTIONS[k + 1];
                            if new_row >= 0
                                && new_col >= 0
                                && new_row < rows as i32
                                && new_col < cols as i32
                            {
                                let new_row = new_row as usize;
                                let new_col = new_col as usize;
                                if grid[new_row][new_col] == 1 {
                                    queue.push_back((new_row, new_col));
                                }
                            }
                        }
                        ans = ans.max(s);
                    }
                }
            }
        }
        ans
    }
}

const DIRECTIONS: &[i32] = &[-1, 0, 1, 0, -1];

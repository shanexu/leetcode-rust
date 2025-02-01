fn main() {
    println!(
        "{}",
        Solution::closed_island(vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0]
        ])
    );
}

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut queue = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if i == 0 || j == 0 || i == m - 1 || j == n - 1 {
                    if grid[i][j] == 0 {
                        queue.push_back((i, j));
                        while let Some((x, y)) = queue.pop_front() {
                            if grid[x][y] == 0 {
                                grid[x][y] = 1;
                                let mut dx = 0;
                                let mut dy = 1;
                                for _ in 0..4 {
                                    let new_x = x as i32 + dx;
                                    let new_y = y as i32 + dy;
                                    (dx, dy) = (dy, -dx);
                                    if new_x >= 0
                                        && new_x < m as i32
                                        && new_y >= 0
                                        && new_y < n as i32
                                    {
                                        let new_x = new_x as usize;
                                        let new_y = new_y as usize;
                                        if grid[new_x][new_y] == 0 {
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
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    ans += 1;
                    queue.push_back((i, j));
                    while let Some((x, y)) = queue.pop_front() {
                        if grid[x][y] == 0 {
                            grid[x][y] = 1;
                            let mut dx = 0;
                            let mut dy = 1;
                            for _ in 0..4 {
                                let new_x = x as i32 + dx;
                                let new_y = y as i32 + dy;
                                (dx, dy) = (dy, -dx);
                                if new_x >= 0 && new_x < m as i32 && new_y >= 0 && new_y < n as i32
                                {
                                    let new_x = new_x as usize;
                                    let new_y = new_y as usize;
                                    if grid[new_x][new_y] == 0 {
                                        queue.push_back((new_x, new_y));
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

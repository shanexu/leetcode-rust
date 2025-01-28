fn main() {
    println!("{}", Solution::find_max_fish(vec![vec![8, 6], vec![2, 6]]));
}

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut queue = VecDeque::new();
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] > 0 && !visited[i][j] {
                    let mut s = 0;
                    queue.push_back((i, j));
                    while let Some((i, j)) = queue.pop_front() {
                        if visited[i][j] {
                            continue;
                        }
                        s += grid[i][j];
                        visited[i][j] = true;
                        for k in 0..4 {
                            let new_i = i as i32 + DIRECTIONS[k];
                            let new_j = j as i32 + DIRECTIONS[k + 1];
                            if new_i >= 0
                                && new_i < rows as i32
                                && new_j >= 0
                                && new_j < cols as i32
                            {
                                let new_i = new_i as usize;
                                let new_j = new_j as usize;
                                if grid[new_i][new_j] > 0 && !visited[new_i][new_j] {
                                    queue.push_back((new_i, new_j));
                                }
                            }
                        }
                    }
                    ans = ans.max(s);
                }
            }
        }
        ans
    }
}

const DIRECTIONS: &[i32] = &[-1, 0, 1, 0, -1];

fn main() {}

struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut rows = vec![-1; m];
        let mut cols = vec![-1; n];
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    if rows[i] != -1 {
                        if !visited[i][rows[i] as usize] {
                            ans += 1;
                            visited[i][rows[i] as usize] = true;
                        }
                        if !visited[i][j] {
                            ans += 1;
                            visited[i][j] = true;
                        }
                    } else {
                        rows[i] = j as i32;
                    }
                    if cols[j] != -1 {
                        if !visited[cols[j] as usize][j] {
                            ans += 1;
                            visited[cols[j] as usize][j] = true;
                        }
                        if !visited[i][j] {
                            ans += 1;
                            visited[i][j] = true;
                        }
                    } else {
                        cols[j] = i as i32;
                    }
                }
            }
        }
        ans
    }
}

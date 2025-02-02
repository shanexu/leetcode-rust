fn main() {
    let grid = vec![vec![1, 0], vec![0, 1]];
    let ret = Solution::count_servers(grid);
    println!("ret={}", ret);
}

struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut rows = vec![UNDEFINED; m];
        let mut cols = vec![UNDEFINED; n];
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    if rows[i] != UNDEFINED {
                        if !visited[i][rows[i]] {
                            ans += 1;
                            visited[i][rows[i]] = true;
                        }
                        ans += 1;
                        visited[i][j] = true;
                    } else {
                        rows[i] = j;
                    }
                    if cols[j] != UNDEFINED {
                        if !visited[cols[j]][j] {
                            ans += 1;
                            visited[cols[j]][j] = true;
                        }
                        if !visited[i][j] {
                            ans += 1;
                            visited[i][j] = true;
                        }
                    } else {
                        cols[j] = i;
                    }
                }
            }
        }
        ans
    }
}

const UNDEFINED: usize = 1000;

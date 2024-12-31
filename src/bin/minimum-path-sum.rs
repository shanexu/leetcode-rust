fn main() {

}

struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut ans = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                let mut min = i32::MAX;
                if i == 0 && j == 0 {
                    min = 0;
                }
                if i > 0 {
                    min = min.min(ans[i - 1][j]);
                }
                if j > 0 {
                    min = min.min(ans[i][j - 1]);
                }
                ans[i][j] = min + grid[i][j];
            }
        }
        ans[n-1][m-1]
    }
}

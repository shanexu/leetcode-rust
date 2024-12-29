fn main() {
    println!(
        "{}",
        Solution::min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]])
    );
}

struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut ans = matrix.clone();
        let n = matrix[0].len();
        let m = matrix.len();
        for i in 1..m {
            for j in 0..n {
                let mut m = i32::MAX;
                if j > 0 {
                    m = m.min(ans[i - 1][j - 1]);
                }
                m = m.min(ans[i - 1][j]);
                if j < n - 1 {
                    m = m.min(ans[i - 1][j + 1]);
                }
                ans[i][j] = m + matrix[i][j];
            }
        }
        *ans[m - 1].iter().min().unwrap()
    }
}

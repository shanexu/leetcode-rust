use leetcode_rust::vec_vec_i32;

fn main() {
    println!(
        "{:?}",
        Solution::matrix_reshape(vec_vec_i32![[1, 2], [3, 4]], 1, 4)
    );
    println!(
        "{:?}",
        Solution::matrix_reshape(vec_vec_i32![[1, 2], [3, 4]], 2, 4)
    );
}

struct Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let r = r as usize;
        let c = c as usize;
        if r * c != m * n {
            return mat;
        }
        let mut ans = vec![vec![0; c]; r];
        for i in 0..m {
            for j in 0..n {
                let t = i * n + j;
                ans[t / c][t % c] = mat[i][j];
            }
        }
        ans
    }
}

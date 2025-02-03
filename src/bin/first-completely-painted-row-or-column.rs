use leetcode_rust::vec_vec_i32;

fn main() {
    println!(
        "{}",
        Solution::first_complete_index(
            vec![2, 8, 7, 4, 1, 3, 5, 6, 9],
            vec_vec_i32![[3, 2, 5], [1, 4, 6], [8, 7, 9]]
        )
    );
}

struct Solution;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut map: Vec<(usize, usize)> = vec![(0, 0); m * n + 1];
        for i in 0..m {
            for j in 0..n {
                map[mat[i][j] as usize] = (i, j);
            }
        }
        let mut rows = vec![0; m];
        let mut cols = vec![0; n];
        for (k, &v) in arr.iter().enumerate() {
            let (i, j) = map[v as usize];
            rows[i] += 1;
            if rows[i] == n {
                return k as i32;
            }
            cols[j] += 1;
            if cols[j] == m {
                return k as i32;
            }
        }
        unreachable!()
    }
}

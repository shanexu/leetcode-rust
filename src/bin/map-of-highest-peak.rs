fn main() {
    println!(
        "{:?}",
        Solution::highest_peak(vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]])
    );
}

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = is_water.len();
        let n = is_water[0].len();
        let mut ans = vec![vec![-1; n]; m];
        let mut queue = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    queue.push_back((i, j));
                    ans[i][j] = 0;
                }
            }
        }
        while let Some((i, j)) = queue.pop_front() {
            for k in 0..4 {
                let new_i = i as i32 + DIRECTIONS[k];
                let new_j = j as i32 + DIRECTIONS[k + 1];
                if new_i >= 0 && new_i < m as i32 && new_j >= 0 && new_j < n as i32 {
                    let new_i = new_i as usize;
                    let new_j = new_j as usize;
                    if ans[new_i][new_j] == -1 {
                        ans[new_i][new_j] = ans[i][j] + 1;
                        queue.push_back((new_i, new_j));
                    }
                }
            }
        }
        ans
    }
}

const DIRECTIONS: &[i32] = &[-1, 0, 1, 0, -1];

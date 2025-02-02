fn main() {
    assert_eq!(
        Solution::check_x_matrix(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        false
    );
}

struct Solution;

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        for i in 0..n {
            for j in 0..n {
                if i == j || i + j == n - 1 {
                    if grid[i][j] == 0 {
                        return false;
                    }
                } else {
                    if grid[i][j] != 0 {
                        return false;
                    }
                }
            }
        }
        true
    }
}

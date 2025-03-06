fn main() {
    println!(
        "{:?}",
        Solution::find_missing_and_repeated_values(vec![
            vec![1, 2, 2],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ])
    );
}

/// 空间复杂度 O(1)
struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(mut grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let mut ans = vec![0; 2];
        let mut missing = (1 + n * n) * n * n / 2;
        for i in 0..n {
            for j in 0..n {
                let v = grid[i][j].abs() as usize;
                let x = (v - 1) / n;
                let y = (v - 1) % n;
                let u = grid[x][y];
                if u < 0 {
                    ans[0] = (x * n + y + 1) as i32
                } else {
                    grid[x][y] = -u;
                    missing -= v;
                }
            }
        }
        ans[1] = missing as i32;
        ans
    }
}

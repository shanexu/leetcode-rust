fn main() {
    println!(
        "{}",
        Solution::grid_game(vec![vec![2, 5, 4], vec![1, 5, 1]])
    );
}

struct Solution;

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let mut sum1: i64 = grid[0].iter().map(|x| *x as i64).sum();
        let mut sum2 = 0;
        let n = grid[0].len();
        let mut ans = i64::MAX;
        for i in 0..n {
            sum1 -= grid[0][i] as i64;
            ans = ans.min(sum1.max(sum2));
            sum2 += grid[1][i] as i64;
        }
        ans
    }
}

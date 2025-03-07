fn main() {
    println!(
        "{}",
        Solution::island_perimeter(vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0],
        ])
    );
}

struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut ans = 0;
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 1 {
                    ans += 4;
                    if i + 1 < rows && grid[i + 1][j] == 1 {
                        ans -= 2;
                    }
                    if j + 1 < cols && grid[i][j + 1] == 1 {
                        ans -= 2;
                    }
                }
            }
        }
        ans
    }
}

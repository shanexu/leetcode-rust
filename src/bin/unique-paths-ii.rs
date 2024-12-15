fn main() {
    println!(
        "{}",
        Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]])
    );
    println!(
        "{}",
        Solution::unique_paths_with_obstacles(vec![vec![1, 0]])
    );
    println!(
        "{}",
        Solution::unique_paths_with_obstacles(vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1],
            vec![0, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 0]
        ])
    )
}

/// 在unique-path Solution2上修改
struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        if obstacle_grid[m - 1][n - 1] == 1 {
            return 0;
        }
        if obstacle_grid[0][0] == 1 {
            return 0;
        }
        let mut memo = vec![vec![0; n]; m];

        memo[m - 1][n - 1] = 1;
        {
            let j = n - 1;
            for i in (0..m - 1).rev() {
                memo[i][j] = if obstacle_grid[i][j] == 1 {
                    0
                } else if obstacle_grid[i + 1][j] == 0 {
                    memo[i + 1][j]
                } else {
                    0
                };
            }
        }
        {
            let i = m - 1;
            for j in (0..n - 1).rev() {
                memo[i][j] = if obstacle_grid[i][j] == 1 {
                    0
                } else if obstacle_grid[i][j + 1] == 0 {
                    memo[i][j + 1]
                } else {
                    0
                };
            }
        }

        for i in (0..m - 1).rev() {
            for j in (0..n - 1).rev() {
                memo[i][j] = if obstacle_grid[i][j] == 1 {
                    0
                } else if obstacle_grid[i + 1][j] == 0 {
                    memo[i + 1][j]
                } else {
                    0
                } + if obstacle_grid[i][j + 1] == 0 {
                    memo[i][j + 1]
                } else {
                    0
                };
            }
        }
        memo[0][0]
    }
}

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

/// 空间复杂度为 O(n)
struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let mut diff = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                let x = grid[i][j];
                diff[(x - 1) as usize / n] += x;
                diff[i] -= (i * n + j + 1) as i32;
            }
        }
        let mut ans = vec![0; 2]; // 0 重复 1 缺失
        let mut r = n;
        for i in 0..n {
            let d = diff[i];
            if d > 0 {
                ans[0] = d;
                r = i;
            } else if d < 0 {
                ans[1] = -d;
                r = i;
            }
        }
        if ans[0] != 0 && ans[1] != 0 {
            return ans;
        }
        let mut std_sum = ((r * n + 1 + r * n + n) * n / 2) as i32;
        diff.fill(0);
        for i in 0..n {
            for j in 0..n {
                let x = grid[i][j];
                let row = (x - 1) as usize / n;
                if row == r {
                    let col = (x - 1) as usize % n;
                    if diff[col] == 0 {
                        diff[col] = 1;
                        std_sum -= x;
                    } else {
                        ans[0] = x;
                    }
                }
            }
        }
        ans[1] = std_sum;
        ans
    }
}

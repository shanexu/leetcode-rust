fn main() {
    println!(
        "{}",
        Solution::min_cut("ababababababababababababcbabababababababababababa".to_string())
    );
    // println!(
    //     "{}",
    //     Solution::min_cut("ccbc".to_string())
    // );
}

struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        fn dfs(start: usize, dp: &Vec<Vec<bool>>, memo: &mut Vec<i32>) -> i32 {
            if start == dp.len() {
                return 0;
            }
            if memo[start] != -1 {
                return memo[start];
            }
            let mut ans = i32::MAX;
            for i in start..dp.len() {
                if dp[start][i] {
                    ans = ans.min(dfs(i + 1, dp, memo) + 1);
                }
            }
            memo[start] = ans;
            ans
        }
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![true; n]; n];
        for i in (0..n).rev() {
            for j in i + 1..n {
                dp[i][j] = s[i] == s[j] && dp[i + 1][j - 1];
            }
        }
        dfs(0, &dp, &mut vec![-1; n]) - 1
    }
}

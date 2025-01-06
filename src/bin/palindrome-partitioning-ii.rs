fn main() {
    // println!(
    //     "{}",
    //     Solution::min_cut("ababababababababababababcbabababababababababababa".to_string())
    // );
    println!("{}", Solution::min_cut("cbcc".to_string()));
}

struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![true; n]; n];
        for i in (0..n).rev() {
            for j in i + 1..n {
                dp[i][j] = s[i] == s[j] && dp[i + 1][j - 1];
            }
        }
        let mut ans = vec![i32::MAX; n];
        for i in 0..n {
            let prev = if i == 0 { 0 } else { ans[i - 1] };
            for j in i..n {
                if dp[i][j] {
                    ans[j] = ans[j].min(prev + 1);
                }
            }
        }
        ans[n - 1] - 1
    }
}

fn main() {
    println!("{:?}", Solution::partition("aab".to_string()));
}

struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn dfs<'a>(
            start: usize,
            s: &'a [u8],
            dp: &Vec<Vec<bool>>,
            current: &mut Vec<&'a [u8]>,
            ans: &mut Vec<Vec<String>>,
        ) {
            if start == s.len() {
                ans.push(
                    current
                        .iter()
                        .map(|&s| String::from_utf8(s.to_vec()).unwrap())
                        .collect(),
                );
                return;
            }
            for i in start..s.len() {
                if dp[start][i] {
                    current.push(&s[start..=i]);
                    dfs(i + 1, s, dp, current, ans);
                    current.pop();
                }
            }
        }

        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![true; n]; n];
        for i in (0..n).rev() {
            for j in (i + 1)..n {
                dp[i][j] = s[i] == s[j] && dp[i + 1][j - 1];
            }
        }
        let mut ans = vec![];
        dfs(0, s, &dp, &mut vec![], &mut ans);
        ans
    }
}

fn main() {
    // println!(
    //     "{}",
    //     Solution2::num_distinct("rabbbit".to_string(), "rabbit".to_string())
    // );
    // println!(
    //     "{}",
    //     Solution2::num_distinct("babgba".to_string(), "ba".to_string())
    // );
    // println!(
    //     "{}",
    //     Solution2::num_distinct("aabbcc".to_string(), "abc".to_string())
    // );
    println!(
        "{}",
        Solution::num_distinct("ddd".to_string(), "dd".to_string())
    );
    println!(
        "{}",
        Solution2::num_distinct("ddd".to_string(), "dd".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        fn helper(s: &[u8], t: &[u8], i: usize, j: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if j == t.len() {
                return 1;
            }
            if i == s.len() {
                return 0;
            }
            if memo[i][j] != -1 {
                return memo[i][j];
            }
            let mut x = 0;
            if s[i] == t[j] {
                x += helper(s, t, i + 1, j + 1, memo);
            }
            x += helper(s, t, i + 1, j, memo);
            memo[i][j] = x;
            x
        }
        let s = s.as_bytes();
        let t = t.as_bytes();
        helper(s, t, 0, 0, &mut vec![vec![-1; t.len()]; s.len()])
    }
}

struct Solution2;

impl Solution2 {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let m = s.len();
        let n = t.len();
        let mut dp = vec![0; m + 1];
        for i in 0..=m {
            dp[i] = 1;
        }
        for j in 1..=n {
            let mut left = 0;
            let mut top_left = dp[0];
            dp[0] = 0;
            for i in 1..=m {
                (top_left, dp[i]) = (
                    dp[i],
                    if s[i - 1] == t[j - 1] {
                        left + top_left
                    } else {
                        left
                    },
                );
                left = dp[i];
            }
        }
        dp[m]
    }
}

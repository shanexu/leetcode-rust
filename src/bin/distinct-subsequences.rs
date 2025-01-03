fn main() {
    println!("{}", Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()));
    println!("{}", Solution::num_distinct("babgbag".to_string(), "bag".to_string()));
}

struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        fn helper(
            s: &[u8],
            t: &[u8],
            i: usize,
            j: usize,
            memo: &mut Vec<Vec<i32>>,
        ) -> i32 {
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
        helper(s, t, 0, 0, &mut vec![vec![-1; t.len()];s.len()])
    }
}

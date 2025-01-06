fn main() {
    println!(
        "{}",
        Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string()
        )
    );
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        fn helper(
            s1: &[u8],
            s2: &[u8],
            s3: &[u8],
            i1: usize,
            i2: usize,
            i3: usize,
            memo: &mut HashMap<(usize, usize, usize), bool>,
        ) -> bool {
            if s1.len() == i1 && s2.len() == i2 && s3.len() == i3 {
                return true;
            }
            if let Some(&v) = memo.get(&(i1, i2, i3)) {
                return v;
            }
            if s1.len() == i1 {
                let ans = s3[i3..] == s2[i2..];
                memo.insert((i1, i2, i3), ans);
                return ans;
            }
            if s2.len() == i2 {
                let ans = s3[i3..] == s1[i1..];
                memo.insert((i1, i2, i3), ans);
                return ans
            }
            if s3[i3] == s1[i1] {
                if helper(s1, s2, s3, i1 + 1, i2, i3 + 1, memo) {
                    memo.insert((i1, i2, i3), true);
                    return true;
                }
            }
            if s3[i3] == s2[i2] {
                if helper(s1, s2, s3, i1, i2 + 1, i3 + 1, memo) {
                    memo.insert((i1, i2, i3), true);
                    return true;
                }
            }
            memo.insert((i1, i2, i3), false);
            false
        }
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        if s3.len() != s1.len() + s2.len() {
            return false;
        }
        helper(s1, s2, s3, 0, 0, 0, &mut HashMap::new())
    }
}

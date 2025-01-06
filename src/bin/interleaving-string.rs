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

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        fn helper(
            s1: &[u8],
            s2: &[u8],
            s3: &[u8],
            i1: usize,
            i2: usize,
            i3: usize,
            memo: &mut Vec<i8>,
        ) -> i8 {
            if s1.len() == i1 && s2.len() == i2 && s3.len() == i3 {
                return 1;
            }
            let idx = s2.len() * i1 + i2 + s1.len() * s2.len() * i3;
            if memo[idx] != -1 {
                return memo[idx];
            }
            if s1.len() == i1 {
                let ok = s3[i3..] == s2[i2..];
                memo[idx] = i8::from(ok);
                return i8::from(ok);
            }
            if s2.len() == i2 {
                let ans = s3[i3..] == s1[i1..];
                memo[idx] = i8::from(ans);
                return i8::from(ans);
            }
            if s3[i3] == s1[i1] {
                let ans = helper(s1, s2, s3, i1 + 1, i2, i3 + 1, memo);
                if ans == 1 {
                    memo[idx] = ans;
                    return ans;
                }
            }
            if s3[i3] == s2[i2] {
                let ans = helper(s1, s2, s3, i1, i2 + 1, i3 + 1, memo);
                if ans == 1 {
                    memo[idx] = ans;
                    return ans;
                }
            }
            memo[idx] = 0;
            0
        }
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        if s3.len() != s1.len() + s2.len() {
            return false;
        }
        helper(
            s1,
            s2,
            s3,
            0,
            0,
            0,
            &mut vec![-1; (s1.len() + 1) * (s2.len() + 1) * (s3.len() + 1)],
        ) == 1
    }
}

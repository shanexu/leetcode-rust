fn main() {
    println!(
        "{}",
        Solution::is_scramble("great".to_string(), "rgeat".to_string())
    );
    println!(
        "{}",
        Solution::is_scramble("reatg".to_string(), "great".to_string())
    );
    println!(
        "{}",
        Solution::is_scramble("abcde".to_string(), "caebd".to_string())
    );
    println!(
        "{}",
        Solution::is_scramble(
            "eebaacbcbcadaaedceaaacadccd".to_string(),
            "eadcaacabaddaceacbceaabeccd".to_string()
        )
    );
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        fn helper<'a>(
            s1: &'a [u8],
            s2: &'a [u8],
            memo: &mut HashMap<(&'a [u8], &'a [u8]), bool>,
        ) -> bool {
            if s1.len() == 1 {
                return s1[0] == s2[0];
            }
            if let Some(&v) = memo.get(&(s1, s2)) {
                return v;
            }
            let mut counter1 = vec![0i8; 26];
            let mut counter1r = vec![0i8; 26];
            let mut counter2 = vec![0i8; 26];
            let n = s1.len();
            for i in 0..(n - 1) {
                let b1 = s1[i];
                let b2 = s2[i];
                let b1r = s1[n - i - 1];
                counter1[(b1 - b'a') as usize] += 1;
                counter1r[(b1r - b'a') as usize] += 1;
                counter2[(b2 - b'a') as usize] += 1;
                if counter1 == counter2 {
                    if helper(&s1[..i + 1], &s2[..i + 1], memo)
                        && helper(&s1[i + 1..], &s2[i + 1..], memo)
                    {
                        memo.insert((s1, s2), true);
                        return true;
                    }
                }
                if counter1r == counter2 {
                    if helper(&s1[n - i - 1..], &s2[..i + 1], memo)
                        && helper(&s1[..n - i - 1], &s2[i + 1..], memo)
                    {
                        memo.insert((s1, s2), true);
                        return true;
                    }
                }
            }
            memo.insert((s1, s2), false);
            false
        }
        helper(s1.as_bytes(), s2.as_bytes(), &mut HashMap::new())
    }
}

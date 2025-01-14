fn main() {
    println!(
        "{:?}",
        Solution::subdomain_visits(vec!["9001 discuss.leetcode.com".to_string()])
    );
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut freq = HashMap::new();
        fn parse_int(s: &[u8]) -> i32 {
            let mut i = 0;
            for &b in s {
                i = i * 10 + (b - b'0') as i32;
            }
            i
        }
        for s in cpdomains.iter() {
            let s = s.as_bytes();
            let mut c = 0;
            for (i, &b) in s.iter().enumerate() {
                if b == b' ' {
                    c = parse_int(&s[..i]);
                    let d = &s[i + 1..];
                    *freq.entry(d).or_insert(0) += c;
                } else if b == b'.' {
                    let d = &s[i + 1..];
                    *freq.entry(d).or_insert(0) += c;
                }
            }
        }
        let mut ans = vec![];
        for (d, c) in freq {
            ans.push(format!("{} {}", c, String::from_utf8(d.to_vec()).unwrap()));
        }
        ans
    }
}
fn main() {
    println!("{:?}", Solution::partition("abbab".to_string()));
}

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn helper<'a>(
            s: &'a [u8],
            memo: &mut HashMap<&'a [u8], HashSet<Vec<String>>>,
        ) -> HashSet<Vec<String>> {
            if s.len() == 0 {
                unreachable!();
            }
            if s.len() == 1 {
                let mut res = HashSet::new();
                res.insert(vec![String::from_utf8_lossy(s).to_string()]);
                return res;
            }
            if let Some(res) = memo.get(s) {
                return res.clone();
            }
            let mut res = HashSet::new();
            res.insert(s.iter().map(|&b| String::from(char::from(b))).collect());
            for k in 1..s.len() {
                let prev = helper(&s[..k], memo);
                let post = helper(&s[k..], memo);
                for vec1 in prev.iter() {
                    for vec2 in post.iter() {
                        let merged: Vec<String> =
                            vec1.iter().cloned().chain(vec2.iter().cloned()).collect();
                        res.insert(merged);
                    }
                }
            }
            if is_palindrome(s) {
                res.insert(vec![String::from_utf8_lossy(s).to_string()]);
            }
            memo.insert(s, res.clone());
            res
        }
        helper(s.as_bytes(), &mut HashMap::new())
            .into_iter()
            .collect()
    }
}

fn is_palindrome(s: &[u8]) -> bool {
    let mut start = 0;
    let mut end = s.len() - 1;
    while start < end {
        if s[start] != s[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    true
}

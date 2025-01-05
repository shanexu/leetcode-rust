fn main() {
    println!(
        "{:?}",
        Solution::license_key_formatting("5F3Z-2e-9-w".to_string(), 4)
    );
    println!(
        "{:?}",
        Solution::license_key_formatting("2-5g-3-J".to_string(), 2)
    );
}

struct Solution2;

impl Solution2 {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let s = s.chars().filter(|&c| c != '-').collect::<Vec<_>>();
        let k = k as usize;
        let n = s.len();
        let mut i = n % k;
        let mut res = String::new();
        if i > 0 {
            res.push_str(&s[..i].iter().collect::<String>());
            res.push('-');
        }
        while i < n {
            res.push_str(&s[i..i + k].iter().collect::<String>());
            res.push('-');
            i += k;
        }
        res.pop();
        res.to_ascii_uppercase()
    }
}

struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let s = s.as_bytes();
        let k = k as usize;
        let mut ans = vec![];
        for &b in s.iter().rev() {
            if b == b'-' {
                continue;
            }
            if !ans.is_empty() && ans.len() % (k + 1) == k {
                ans.push(b'-');
            }
            ans.push(b.to_ascii_uppercase());
        }
        ans.reverse();
        String::from_utf8(ans).unwrap()
    }
}
